import { invoke, Channel } from "@tauri-apps/api/core";

export type ReqRespEvent =
  | { type: "listening"; addr: string }
  | { type: "connected"; peerId: string }
  | { type: "disconnected"; peerId: string }
  | { type: "ping"; peerId: string; rttMs: number }
  | { type: "identified"; peerId: string; agentVersion: string }
  | { type: "inboundRequest"; requestId: number; peerId: string; name: string; age: number }
  | { type: "response"; peerId: string; message: string }
  | { type: "requestFailed"; peerId: string; error: string }
  | { type: "error"; message: string };

export type ReqRespCommand =
  | { command: "dial"; addr: string }
  | { command: "disconnect"; peerId: string }
  | { command: "sendRequest"; peerId: string; name: string; age: number }
  | { command: "sendResponse"; requestId: number; message: string }
  | { command: "stop" };

export function startReqRespNode(onEvent: (event: ReqRespEvent) => void) {
  const channel = new Channel<ReqRespEvent>();
  channel.onmessage = onEvent;
  return invoke<string>("start_reqresp_node", { onEvent: channel });
}

export function sendReqRespCommand(cmd: ReqRespCommand) {
  return invoke("send_reqresp_command", { cmd });
}
