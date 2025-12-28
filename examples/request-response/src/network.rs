use crate::{MyBehaviour, MyBehaviourEvent, MyRequest, MyResponse};
use anyhow::Result;
use libp2p::{
    Multiaddr, PeerId, Swarm,
    futures::StreamExt,
    identify,
    request_response::{self, Message, OutboundRequestId, ResponseChannel},
};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use tokio::sync::{mpsc, oneshot};

pub struct InboundRequest {
    pub peer: PeerId,
    pub request: MyRequest,
    pub channel: ResponseChannel<MyResponse>,
}

pub enum Command {
    Dial(Multiaddr),
    SendRequest {
        peer_id: PeerId,
        request: MyRequest,
        sender: oneshot::Sender<Result<MyResponse>>,
    },
    SendResponse {
        channel: ResponseChannel<MyResponse>,
        response: MyResponse,
    },
}

pub struct EventLoop {
    swarm: Swarm<MyBehaviour>,
    command_rx: mpsc::Receiver<Command>,
    event_tx: mpsc::Sender<InboundRequest>,
    pending_requests: HashMap<OutboundRequestId, oneshot::Sender<Result<MyResponse>>>,
    peers: Arc<RwLock<HashSet<PeerId>>>,
}

impl EventLoop {
    pub fn new(
        swarm: Swarm<MyBehaviour>,
        command_rx: mpsc::Receiver<Command>,
        event_tx: mpsc::Sender<InboundRequest>,
        peers: Arc<RwLock<HashSet<PeerId>>>,
    ) -> Self {
        Self {
            swarm,
            command_rx,
            event_tx,
            pending_requests: HashMap::new(),
            peers,
        }
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => self.handle_event(event).await,
                command = self.command_rx.recv() => match command {
                    Some(cmd) => self.handle_command(cmd),
                    None => return,
                },
            }
        }
    }

    async fn handle_event(&mut self, event: libp2p::swarm::SwarmEvent<MyBehaviourEvent>) {
        match event {
            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                tracing::info!("Listening on {address}");
            }
            libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                self.peers.write().unwrap().insert(peer_id);
                tracing::info!("Connected to {peer_id}");
            }
            libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. } => {
                self.peers.write().unwrap().remove(&peer_id);
                tracing::info!("Disconnected from {peer_id}");
            }
            libp2p::swarm::SwarmEvent::Behaviour(event) => match event {
                MyBehaviourEvent::Ping(event) => {
                    tracing::info!("Ping: {event:?}");
                }
                MyBehaviourEvent::Identify(identify::Event::Received { peer_id, info, .. }) => {
                    tracing::info!(
                        "Identified {peer_id}:\n  protocol: {}\n  agent: {}\n  addrs: {:?}",
                        info.protocol_version,
                        info.agent_version,
                        info.listen_addrs
                    );
                }
                MyBehaviourEvent::Identify(_) => {}
                MyBehaviourEvent::RequestResponse(event) => match event {
                    request_response::Event::Message { peer, message, .. } => match message {
                        Message::Request {
                            request, channel, ..
                        } => {
                            tracing::info!("Request from {peer}: {request:?}");
                            let _ = self
                                .event_tx
                                .send(InboundRequest {
                                    peer,
                                    request,
                                    channel,
                                })
                                .await;
                        }
                        Message::Response {
                            request_id,
                            response,
                        } => {
                            if let Some(sender) = self.pending_requests.remove(&request_id) {
                                let _ = sender.send(Ok(response));
                            }
                        }
                    },
                    request_response::Event::OutboundFailure {
                        request_id, error, ..
                    } => {
                        if let Some(sender) = self.pending_requests.remove(&request_id) {
                            let _ = sender.send(Err(error.into()));
                        }
                    }
                    _ => {}
                },
            },
            _ => {}
        }
    }

    fn handle_command(&mut self, command: Command) {
        match command {
            Command::Dial(addr) => {
                if let Err(e) = self.swarm.dial(addr.clone()) {
                    tracing::error!("Failed to dial {addr}: {e}");
                }
            }
            Command::SendRequest {
                peer_id,
                request,
                sender,
            } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_request(&peer_id, request);
                self.pending_requests.insert(request_id, sender);
            }
            Command::SendResponse { channel, response } => {
                let _ = self
                    .swarm
                    .behaviour_mut()
                    .request_response
                    .send_response(channel, response);
            }
        }
    }
}

#[derive(Clone)]
pub struct Client {
    command_tx: mpsc::Sender<Command>,
}

impl Client {
    pub fn new(command_tx: mpsc::Sender<Command>) -> Self {
        Self { command_tx }
    }

    pub async fn dial(&self, addr: Multiaddr) -> Result<()> {
        self.command_tx.send(Command::Dial(addr)).await?;
        Ok(())
    }

    pub async fn send_request(&self, peer_id: PeerId, request: MyRequest) -> Result<MyResponse> {
        let (tx, rx) = oneshot::channel();
        self.command_tx
            .send(Command::SendRequest {
                peer_id,
                request,
                sender: tx,
            })
            .await?;
        rx.await?
    }

    pub async fn send_response(
        &self,
        channel: ResponseChannel<MyResponse>,
        response: MyResponse,
    ) -> Result<()> {
        self.command_tx
            .send(Command::SendResponse { channel, response })
            .await?;
        Ok(())
    }
}
