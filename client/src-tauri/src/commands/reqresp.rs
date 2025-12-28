use std::time::Duration;

use anyhow::Result;
use libp2p::{
    futures::StreamExt,
    identify, noise, ping,
    request_response::{self, Message, ProtocolSupport, ResponseChannel},
    swarm::{self, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, StreamProtocol, Swarm, SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, AppHandle, Manager, State};
use tokio::{
    select,
    sync::{mpsc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRequest {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyResponse {
    pub message: String,
}

#[derive(swarm::NetworkBehaviour)]
pub struct ReqRespBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    request_response: request_response::cbor::Behaviour<MyRequest, MyResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "camelCase")]
pub enum ReqRespCommand {
    Dial {
        addr: Multiaddr,
    },
    #[serde(rename_all = "camelCase")]
    Disconnect {
        peer_id: PeerId,
    },
    #[serde(rename_all = "camelCase")]
    SendRequest {
        peer_id: PeerId,
        name: String,
        age: u32,
    },
    #[serde(rename_all = "camelCase")]
    SendResponse {
        request_id: u64,
        message: String,
    },
    Stop,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ReqRespEvent {
    Listening {
        addr: String,
    },
    #[serde(rename_all = "camelCase")]
    Connected {
        peer_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Disconnected {
        peer_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Ping {
        peer_id: String,
        rtt_ms: u128,
    },
    #[serde(rename_all = "camelCase")]
    Identified {
        peer_id: String,
        agent_version: String,
    },
    #[serde(rename_all = "camelCase")]
    InboundRequest {
        request_id: u64,
        peer_id: String,
        name: String,
        age: u32,
    },
    #[serde(rename_all = "camelCase")]
    Response {
        peer_id: String,
        message: String,
    },
    #[serde(rename_all = "camelCase")]
    RequestFailed {
        peer_id: String,
        error: String,
    },
    Error {
        message: String,
    },
}

type PendingChannels =
    std::sync::Arc<Mutex<std::collections::HashMap<u64, ResponseChannel<MyResponse>>>>;

pub struct ReqRespState {
    tx: mpsc::Sender<ReqRespCommand>,
    pending_channels: PendingChannels,
}

fn create_swarm() -> Result<Swarm<ReqRespBehaviour>> {
    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|keypair| ReqRespBehaviour {
            ping: ping::Behaviour::default(),
            identify: identify::Behaviour::new(
                identify::Config::new("/swarmbook/1.0.0".into(), keypair.public())
                    .with_interval(Duration::from_secs(30)),
            ),
            request_response: request_response::cbor::Behaviour::new(
                [(
                    StreamProtocol::new("/swarmbook/req-resp/1.0.0"),
                    ProtocolSupport::Full,
                )],
                request_response::Config::default().with_request_timeout(Duration::from_secs(600)),
            ),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();
    Ok(swarm)
}

#[tauri::command]
pub async fn start_reqresp_node(
    app: AppHandle,
    on_event: Channel<ReqRespEvent>,
) -> Result<String, String> {
    let state = app.try_state::<Mutex<ReqRespState>>();
    let mut swarm = create_swarm().map_err(|e| e.to_string())?;
    let peer_id = swarm.local_peer_id().to_string();
    let (tx, mut rx) = mpsc::channel::<ReqRespCommand>(32);
    let pending_channels: PendingChannels =
        std::sync::Arc::new(Mutex::new(std::collections::HashMap::new()));
    let pending_channels_clone = pending_channels.clone();

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .map_err(|e| e.to_string())?;

    if let Some(state) = state {
        let mut s = state.lock().await;
        s.tx = tx;
        s.pending_channels = pending_channels;
    } else {
        app.manage(Mutex::new(ReqRespState {
            tx,
            pending_channels,
        }));
    }

    let mut request_counter: u64 = 0;
    let mut pending_requests: std::collections::HashMap<
        request_response::OutboundRequestId,
        PeerId,
    > = std::collections::HashMap::new();

    tokio::spawn(async move {
        loop {
            select! {
                command = rx.recv() => {
                    if let Some(command) = command {
                        match command {
                            ReqRespCommand::Dial { addr } => { let _ = swarm.dial(addr); }
                            ReqRespCommand::Disconnect { peer_id } => { let _ = swarm.disconnect_peer_id(peer_id); }
                            ReqRespCommand::SendRequest { peer_id, name, age } => {
                                let req_id = swarm.behaviour_mut().request_response.send_request(&peer_id, MyRequest { name, age });
                                pending_requests.insert(req_id, peer_id);
                            }
                            ReqRespCommand::SendResponse { request_id, message } => {
                                if let Some(channel) = pending_channels_clone.lock().await.remove(&request_id) {
                                    let _ = swarm.behaviour_mut().request_response.send_response(channel, MyResponse { message });
                                }
                            }
                            ReqRespCommand::Stop => break,
                        }
                    }
                }
                event = swarm.select_next_some() => {
                    let evt = match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            ReqRespEvent::Listening { addr: address.to_string() }
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            ReqRespEvent::Connected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            ReqRespEvent::Disconnected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::Behaviour(event) => match event {
                            ReqRespBehaviourEvent::Ping(ping::Event { peer, result, .. }) => {
                                match result {
                                    Ok(rtt) => ReqRespEvent::Ping { peer_id: peer.to_string(), rtt_ms: rtt.as_millis() },
                                    Err(e) => ReqRespEvent::Error { message: e.to_string() },
                                }
                            }
                            ReqRespBehaviourEvent::Identify(identify::Event::Received { peer_id, info, .. }) => {
                                ReqRespEvent::Identified { peer_id: peer_id.to_string(), agent_version: info.agent_version }
                            }
                            ReqRespBehaviourEvent::Identify(_) => continue,
                            ReqRespBehaviourEvent::RequestResponse(event) => match event {
                                request_response::Event::Message { peer, message, .. } => match message {
                                    Message::Request { request, channel, .. } => {
                                        request_counter += 1;
                                        pending_channels_clone.lock().await.insert(request_counter, channel);
                                        ReqRespEvent::InboundRequest {
                                            request_id: request_counter,
                                            peer_id: peer.to_string(),
                                            name: request.name,
                                            age: request.age,
                                        }
                                    }
                                    Message::Response { request_id, response } => {
                                        let peer_id = pending_requests.remove(&request_id).map(|p| p.to_string()).unwrap_or_default();
                                        ReqRespEvent::Response { peer_id, message: response.message }
                                    }
                                },
                                request_response::Event::OutboundFailure { request_id, error, .. } => {
                                    let peer_id = pending_requests.remove(&request_id).map(|p| p.to_string()).unwrap_or_default();
                                    ReqRespEvent::RequestFailed { peer_id, error: error.to_string() }
                                }
                                _ => continue,
                            },
                        },
                        _ => continue,
                    };
                    let _ = on_event.send(evt);
                }
            }
        }
    });

    Ok(peer_id)
}

#[tauri::command]
pub async fn send_reqresp_command(
    state: State<'_, Mutex<ReqRespState>>,
    cmd: ReqRespCommand,
) -> tauri::Result<()> {
    state.lock().await.tx.send(cmd).await.ok();
    Ok(())
}
