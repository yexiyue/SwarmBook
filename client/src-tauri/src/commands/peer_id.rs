use libp2p::identity::Keypair;
use serde::Serialize;

#[derive(Serialize)]
pub struct PeerIdResult {
    pub peer_id: String,
    pub public_key_hex: String,
    pub key_type: String,
}

#[tauri::command]
pub fn generate_peer_id(key_type: &str) -> Result<PeerIdResult, String> {
    let keypair = match key_type {
        "ed25519" => Keypair::generate_ed25519(),
        "secp256k1" => Keypair::generate_secp256k1(),
        "ecdsa" => Keypair::generate_ecdsa(),
        _ => return Err(format!("Unsupported key type: {}", key_type)),
    };

    let peer_id = keypair.public().to_peer_id();
    let public_key_bytes = keypair.public().encode_protobuf();

    Ok(PeerIdResult {
        peer_id: peer_id.to_string(),
        public_key_hex: hex::encode(&public_key_bytes),
        key_type: key_type.to_string(),
    })
}
