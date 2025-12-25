use anyhow::Result;
use libp2p::{Multiaddr, SwarmBuilder, futures::StreamExt, noise, ping, tcp, yamux};
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let keypair = libp2p::identity::Keypair::generate_ed25519();
    let peer_id = keypair.public().to_peer_id();

    tracing::info!("Local peer id: {:?}", peer_id);

    let mut swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_keypair| ping::Behaviour::default())?
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
                tracing::info!("Listening on {:?}", address);
            }
            libp2p::swarm::SwarmEvent::Behaviour(event) => {
                tracing::info!("{:?}", event);
            }
            other => {
                tracing::warn!("Other {:?}", other)
            }
        }
    }
}
