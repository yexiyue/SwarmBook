import { invoke, Channel } from "@tauri-apps/api/core";

export type PingEvent =
  | { type: "listening"; addr: string }
  | { type: "connected"; peerId: string }
  | { type: "disconnected"; peerId: string }
  | { type: "ping"; peerId: string; rttMs: number }
  | { type: "error"; message: string };

export type PingCommand =
  | { command: "dial"; addr: string }
  | { command: "disconnect"; addr: string }
  | { command: "stop" };

export function startPingNode(onEvent: (event: PingEvent) => void) {
  const channel = new Channel<PingEvent>();
  channel.onmessage = onEvent;
  return invoke<string>("start_ping_node", { onEvent: channel });
}

export function sendPingCommand(cmd: PingCommand) {
  return invoke("send_ping_command", { cmd });
}
