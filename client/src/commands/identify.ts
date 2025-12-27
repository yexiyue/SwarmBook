import { invoke, Channel } from "@tauri-apps/api/core";

export type IdentifyEvent =
  | { type: "listening"; addr: string }
  | { type: "connected"; peerId: string }
  | { type: "disconnected"; peerId: string }
  | { type: "ping"; peerId: string; rttMs: number }
  | {
      type: "identified";
      peerId: string;
      protocolVersion: string;
      agentVersion: string;
      listenAddrs: string[];
    }
  | { type: "error"; message: string };

export type IdentifyCommand =
  | { command: "dial"; addr: string }
  | { command: "disconnect"; addr: string }
  | { command: "stop" };

export function startIdentifyNode(onEvent: (event: IdentifyEvent) => void) {
  const channel = new Channel<IdentifyEvent>();
  channel.onmessage = onEvent;
  return invoke<string>("start_identify_node", { onEvent: channel });
}

export function sendIdentifyCommand(cmd: IdentifyCommand) {
  return invoke("send_identify_command", { cmd });
}
