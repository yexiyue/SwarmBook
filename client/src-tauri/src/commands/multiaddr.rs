use libp2p::{multiaddr::Protocol, Multiaddr};
use serde::Serialize;

/// 协议层级
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ProtocolLayer {
    /// 网络层 (IP)
    Network,
    /// 传输层 (TCP/UDP)
    Transport,
    /// 安全层 (Noise/TLS)
    Security,
    /// 多路复用层 (Yamux/Mplex)
    Muxer,
    /// 应用层 (WebSocket, HTTP, etc.)
    Application,
    /// 身份层 (PeerId)
    Identity,
    /// 中继层
    Relay,
}

/// 解析后的协议组件
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolComponent {
    /// 协议名称
    pub name: String,
    /// 协议值（如果有）
    pub value: Option<String>,
    /// 协议层级
    pub layer: ProtocolLayer,
    /// 原始字符串表示
    pub raw: String,
}

/// 解析结果
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedMultiaddr {
    /// 原始输入
    pub input: String,
    /// 解析后的协议组件列表
    pub components: Vec<ProtocolComponent>,
    /// 是否有效
    pub valid: bool,
    /// 错误信息（如果有）
    pub error: Option<String>,
}

/// 根据协议确定其层级
fn get_protocol_layer(protocol: &Protocol) -> ProtocolLayer {
    match protocol {
        // 网络层
        Protocol::Ip4(_)
        | Protocol::Ip6(_)
        | Protocol::Dns(_)
        | Protocol::Dns4(_)
        | Protocol::Dns6(_)
        | Protocol::Dnsaddr(_) => ProtocolLayer::Network,

        // 传输层
        Protocol::Tcp(_) | Protocol::Udp(_) | Protocol::Dccp(_) | Protocol::Sctp(_) => {
            ProtocolLayer::Transport
        }

        // 安全层
        Protocol::Tls | Protocol::Noise => ProtocolLayer::Security,

        // 应用层
        Protocol::Ws(_)
        | Protocol::Wss(_)
        | Protocol::Http
        | Protocol::Https
        | Protocol::QuicV1
        | Protocol::WebRTC
        | Protocol::WebRTCDirect
        | Protocol::Certhash(_) => ProtocolLayer::Application,

        // 身份层
        Protocol::P2p(_) => ProtocolLayer::Identity,

        // 中继层
        Protocol::P2pCircuit => ProtocolLayer::Relay,

        // 其他默认为应用层
        _ => ProtocolLayer::Application,
    }
}

/// 将协议转换为组件
fn protocol_to_component(protocol: Protocol) -> ProtocolComponent {
    let layer = get_protocol_layer(&protocol);
    let name = protocol.tag().to_string();
    let raw = format!("{}", protocol);

    let value = match &protocol {
        Protocol::Ip4(addr) => Some(addr.to_string()),
        Protocol::Ip6(addr) => Some(addr.to_string()),
        Protocol::Dns(n) | Protocol::Dns4(n) | Protocol::Dns6(n) | Protocol::Dnsaddr(n) => {
            Some(n.to_string())
        }
        Protocol::Tcp(port) | Protocol::Udp(port) | Protocol::Dccp(port) | Protocol::Sctp(port) => {
            Some(port.to_string())
        }
        Protocol::Ws(path) | Protocol::Wss(path) => {
            let s = path.to_string();
            if s == "/" {
                None
            } else {
                Some(s)
            }
        }
        Protocol::Certhash(hash) => Some(format!("{:?}", hash)),
        Protocol::P2p(peer_id) => Some(peer_id.to_string()),
        _ => None,
    };

    ProtocolComponent {
        name,
        value,
        layer,
        raw,
    }
}

#[tauri::command]
pub fn parse_multiaddr(input: String) -> ParsedMultiaddr {
    match input.parse::<Multiaddr>() {
        Ok(addr) => {
            let components: Vec<ProtocolComponent> =
                addr.iter().map(protocol_to_component).collect();

            ParsedMultiaddr {
                input,
                components,
                valid: true,
                error: None,
            }
        }
        Err(e) => ParsedMultiaddr {
            input,
            components: vec![],
            valid: false,
            error: Some(e.to_string()),
        },
    }
}
