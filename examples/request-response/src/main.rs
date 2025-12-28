use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use libp2p::{
    Multiaddr, PeerId, StreamProtocol, SwarmBuilder, identify, noise, ping,
    request_response::{self, ProtocolSupport},
    swarm, tcp, yamux,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing_subscriber::fmt::writer::MakeWriterExt;

mod network;
use network::{Client, EventLoop, InboundRequest};

#[derive(swarm::NetworkBehaviour)]
pub struct MyBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    request_response: request_response::cbor::Behaviour<MyRequest, MyResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyRequest {
    pub name: String,
    pub age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyResponse {
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Log to temp file to avoid interfering with interaction and Tauri hot reload
    let theme = ColorfulTheme::default();
    let log_path = std::env::current_dir()
        .unwrap()
        .join("request-response.log");
    let file = std::fs::File::create(&log_path)?;
    println!("Logging to: {}", log_path.display());
    tracing_subscriber::fmt()
        .with_writer(file.with_max_level(tracing::Level::INFO))
        .with_ansi(false)
        .init();

    let keypair = libp2p::identity::Keypair::generate_ed25519();
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

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let (command_tx, command_rx) = mpsc::channel(32);
    let (event_tx, mut event_rx) = mpsc::channel::<InboundRequest>(32);
    let peers: Arc<RwLock<HashSet<PeerId>>> = Arc::new(RwLock::new(HashSet::new()));
    let client = Client::new(command_tx);

    let mut event_loop = EventLoop::new(swarm, command_rx, event_tx, peers.clone());
    tokio::spawn(async move { event_loop.run().await });

    // Initial dial prompt
    println!("Enter address to dial (or press Enter to skip):");
    let addr: String = Input::new()
        .with_prompt("Multiaddr")
        .allow_empty(true)
        .interact_text()?;
    if !addr.is_empty() {
        if let Ok(addr) = addr.parse::<Multiaddr>() {
            client.dial(addr).await?;
            println!("Dialing... waiting for connection");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    // Main interaction loop - all interaction happens here
    loop {
        // First, drain any pending inbound requests
        while let Ok(req) = event_rx.try_recv() {
            handle_inbound_request(&client, req).await;
        }

        let peer_list: Vec<_> = peers.read().unwrap().iter().copied().collect();
        let mut options = vec!["Dial a peer".to_string()];
        for p in &peer_list {
            options.push(format!("Send to {}", &p.to_string()[..8]));
        }

        options.push("Skip".to_string());
        options.push("Exit".to_string());

        println!("\n--- Connected peers: {} ---", peer_list.len());
        let selection = Select::with_theme(&theme)
            .with_prompt("Action")
            .items(&options)
            .default(0)
            .interact()?;

        if selection == 0 {
            let addr: String = Input::with_theme(&theme)
                .with_prompt("Multiaddr")
                .interact_text()?;
            if let Ok(addr) = addr.parse::<Multiaddr>() {
                client.dial(addr).await?;
                println!("Dialing... waiting for connection");
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        } else if selection == options.len() - 2 {
            continue;
        } else if selection == options.len() - 1 {
            break;
        } else {
            let peer = peer_list[selection - 1];
            let name: String = Input::with_theme(&theme)
                .with_prompt("Name")
                .interact_text()?;
            let age: u32 = Input::with_theme(&theme)
                .with_prompt("Age")
                .interact_text()?;
            match client.send_request(peer, MyRequest { name, age }).await {
                Ok(resp) => println!("✅ Response: {}", resp.message),
                Err(e) => println!("❌ Error: {e}"),
            }
        }
    }
    Ok(())
}

async fn handle_inbound_request(client: &Client, req: InboundRequest) {
    let InboundRequest {
        peer,
        request,
        channel,
    } = req;
    println!(
        "\n📨 Request from {}: {:?}",
        &peer.to_string()[..8],
        request
    );
    let response: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Your response")
        .interact_text()
        .unwrap_or_default();
    let _ = client
        .send_response(channel, MyResponse { message: response })
        .await;
}
