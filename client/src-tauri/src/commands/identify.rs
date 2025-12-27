use std::time::Duration;

use anyhow::Result;
use libp2p::{
    futures::StreamExt, identify, noise, ping, swarm, swarm::SwarmEvent, tcp, yamux, Multiaddr,
    PeerId, Swarm, SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, AppHandle, Manager, State};
use tokio::{
    select,
    sync::{mpsc, Mutex},
};

#[derive(swarm::NetworkBehaviour)]
pub struct IdentifyBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "addr")]
pub enum IdentifyCommand {
    Dial(Multiaddr),
    Disconnect(PeerId),
    Stop,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum IdentifyEvent {
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
        protocol_version: String,
        agent_version: String,
        listen_addrs: Vec<String>,
    },
    Error {
        message: String,
    },
}

pub struct IdentifySwarmState(pub mpsc::Sender<IdentifyCommand>);

fn create_identify_swarm() -> Result<Swarm<IdentifyBehaviour>> {
    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|keypair| IdentifyBehaviour {
            ping: ping::Behaviour::default(),
            identify: identify::Behaviour::new(
                identify::Config::new("/swarmbook/0.1.0".into(), keypair.public())
                    .with_push_listen_addr_updates(true)
                    .with_agent_version("/swarmbook/0.1.0".into())
                    .with_interval(Duration::from_secs(3)),
            ),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();
    Ok(swarm)
}

#[tauri::command]
pub async fn start_identify_node(
    app: AppHandle,
    on_event: Channel<IdentifyEvent>,
) -> Result<String, String> {
    let state = app.try_state::<Mutex<IdentifySwarmState>>();
    let mut swarm = create_identify_swarm().map_err(|e| e.to_string())?;
    let peer_id = swarm.local_peer_id().to_string();
    let (tx, mut rx) = mpsc::channel::<IdentifyCommand>(1);

    swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .map_err(|e| e.to_string())?;

    if let Some(state) = state {
        state.lock().await.0 = tx;
    } else {
        app.manage(Mutex::new(IdentifySwarmState(tx)));
    }

    tokio::spawn(async move {
        loop {
            select! {
                command = rx.recv() => {
                    if let Some(command) = command {
                        match command {
                            IdentifyCommand::Dial(addr) => { let _ = swarm.dial(addr); }
                            IdentifyCommand::Disconnect(peer_id) => { let _ = swarm.disconnect_peer_id(peer_id); }
                            IdentifyCommand::Stop => break,
                        }
                    }
                }
                event = swarm.select_next_some() => {
                    let identify_event = match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            IdentifyEvent::Listening { addr: address.to_string() }
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            IdentifyEvent::Connected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            IdentifyEvent::Disconnected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::Behaviour(event) => match event {
                            IdentifyBehaviourEvent::Ping(ping::Event { peer, result, .. }) => {
                                match result {
                                    Ok(rtt) => IdentifyEvent::Ping { peer_id: peer.to_string(), rtt_ms: rtt.as_millis() },
                                    Err(e) => IdentifyEvent::Error { message: e.to_string() },
                                }
                            }
                            IdentifyBehaviourEvent::Identify(identify::Event::Received { peer_id, info, .. }) => {
                                IdentifyEvent::Identified {
                                    peer_id: peer_id.to_string(),
                                    protocol_version: info.protocol_version,
                                    agent_version: info.agent_version,
                                    listen_addrs: info.listen_addrs.iter().map(|a| a.to_string()).collect(),
                                }
                            }
                            IdentifyBehaviourEvent::Identify(_) => continue,
                        },
                        _ => continue,
                    };
                    let _ = on_event.send(identify_event);
                }
            }
        }
    });

    Ok(peer_id)
}

#[tauri::command]
pub async fn send_identify_command(
    state: State<'_, Mutex<IdentifySwarmState>>,
    cmd: IdentifyCommand,
) -> tauri::Result<()> {
    state.lock().await.0.send(cmd).await.ok();
    Ok(())
}
