use libp2p::identity;

fn main() {
    let keypair = identity::Keypair::generate_ed25519();
    let data = keypair.to_protobuf_encoding();
    println!("{data:?}");
    let peer_id = keypair.public().to_peer_id();

    println!("PeerId: {peer_id}");
}
