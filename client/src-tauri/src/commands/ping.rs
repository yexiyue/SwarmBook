use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use anyhow::Result;
use libp2p::{
    futures::StreamExt, noise, ping, swarm::SwarmEvent, tcp, yamux, Multiaddr, PeerId, Swarm,
    SwarmBuilder,
};
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, AppHandle, Manager, State};
use tokio::{
    select,
    sync::{mpsc, Mutex},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "command", content = "addr")]
pub enum PingCommand {
    Dial(Multiaddr),
    Disconnect(PeerId),
    Stop,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PingEvent {
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
    Error {
        message: String,
    },
}

pub struct PingSwarm {
    swarm: Swarm<ping::Behaviour>,
}

impl Deref for PingSwarm {
    type Target = Swarm<ping::Behaviour>;
    fn deref(&self) -> &Self::Target {
        &self.swarm
    }
}

impl DerefMut for PingSwarm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.swarm
    }
}

impl PingSwarm {
    pub fn new() -> Result<Self> {
        let swarm = SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_behaviour(|_| ping::Behaviour::default())?
            .with_swarm_config(|cfg| {
                cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX))
            })
            .build();
        Ok(Self { swarm })
    }
}

#[tauri::command]
pub async fn start_ping_node(
    app: AppHandle,
    on_event: Channel<PingEvent>,
) -> Result<String, String> {
    let state_tx = app.try_state::<Mutex<mpsc::Sender<PingCommand>>>();
    let mut ping_swarm = PingSwarm::new().map_err(|e| e.to_string())?;
    let peer_id = ping_swarm.local_peer_id().to_string();
    let (tx, mut rx) = mpsc::channel::<PingCommand>(1);
    // 使用 0 端口让系统自动分配可用端口
    ping_swarm
        .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
        .map_err(|e| e.to_string())?;

    if let Some(state_tx) = state_tx {
        *state_tx.lock().await = tx;
    } else {
        app.manage(Mutex::new(tx));
    }

    tokio::spawn(async move {
        loop {
            select! {
                command=rx.recv()=>{
                    if let Some(command)=command{
                        match command{
                            PingCommand::Dial(multiaddr) => {
                                let _ = ping_swarm.dial(multiaddr);
                            },
                            PingCommand::Disconnect(peer_id) => {
                                let _ = ping_swarm.disconnect_peer_id(peer_id);
                            },
                            PingCommand::Stop => break,
                        }
                    }
                }
                event = ping_swarm.select_next_some() => {
                    let ping_event = match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            PingEvent::Listening { addr: address.to_string() }
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            PingEvent::Connected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            PingEvent::Disconnected { peer_id: peer_id.to_string() }
                        }
                        SwarmEvent::Behaviour(ping::Event { peer, result, .. }) => {
                            match result {
                                Ok(rtt) => PingEvent::Ping { peer_id: peer.to_string(), rtt_ms: rtt.as_millis() },
                                Err(e) => PingEvent::Error { message: e.to_string() },
                            }
                        }
                        _ => continue,
                    };
                    let _ = on_event.send(ping_event);
                }
            }
        }
    });

    Ok(peer_id)
}

#[tauri::command]
pub async fn send_ping_command(
    cmd_tx: State<'_, Mutex<mpsc::Sender<PingCommand>>>,
    cmd: PingCommand,
) -> tauri::Result<()> {
    cmd_tx.lock().await.send(cmd).await.ok();
    Ok(())
}
