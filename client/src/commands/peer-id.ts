import { invoke } from "@tauri-apps/api/core";

export interface PeerIdResult {
  peer_id: string;
  public_key_hex: string;
  key_type: string;
}

export type KeyType = "ed25519" | "secp256k1" | "ecdsa";

export function generatePeerId(keyType: KeyType) {
  return invoke<PeerIdResult>("generate_peer_id", { keyType });
}
