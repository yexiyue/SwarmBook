import { invoke } from "@tauri-apps/api/core";

/** 协议层级 */
export type ProtocolLayer =
  | "network"      // 网络层 (IP)
  | "transport"    // 传输层 (TCP/UDP)
  | "security"     // 安全层 (Noise/TLS)
  | "muxer"        // 多路复用层 (Yamux/Mplex)
  | "application"  // 应用层 (WebSocket, HTTP, etc.)
  | "identity"     // 身份层 (PeerId)
  | "relay";       // 中继层

/** 解析后的协议组件 */
export interface ProtocolComponent {
  /** 协议名称 */
  name: string;
  /** 协议值（如果有） */
  value: string | null;
  /** 协议层级 */
  layer: ProtocolLayer;
  /** 原始字符串表示 */
  raw: string;
}

/** 解析结果 */
export interface ParsedMultiaddr {
  /** 原始输入 */
  input: string;
  /** 解析后的协议组件列表 */
  components: ProtocolComponent[];
  /** 是否有效 */
  valid: boolean;
  /** 错误信息（如果有） */
  error: string | null;
}

/** 解析 Multiaddr 字符串 */
export function parseMultiaddr(input: string) {
  return invoke<ParsedMultiaddr>("parse_multiaddr", { input });
}
