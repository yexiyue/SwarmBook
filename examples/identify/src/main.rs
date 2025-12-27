use anyhow::Result;
use libp2p::{
    Multiaddr, SwarmBuilder, futures::StreamExt, identify, noise, ping, swarm, tcp, yamux,
};
use std::{env, time::Duration};

#[derive(swarm::NetworkBehaviour)]
pub struct MyBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();
    tracing::info!("Local peer id: {peer_id}");

    let mut swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|keypair| MyBehaviour {
            ping: ping::Behaviour::default(),
            identify: identify::Behaviour::new(
                identify::Config::new("/my-app/1.0.0".into(), keypair.public())
                    .with_push_listen_addr_updates(true)
                    .with_agent_version("/my-app/0.1.0".into())
                    .with_interval(Duration::from_secs(3)),
            ),
        })?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        tracing::info!("Dialed {addr}");
    }

    loop {
        match swarm.select_next_some().await {
            libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                tracing::info!("Listening on {address}");
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
                MyBehaviourEvent::Identify(event) => {
                    tracing::debug!("Identify: {event:?}");
                }
            },
            _ => {}
        }
    }
}
