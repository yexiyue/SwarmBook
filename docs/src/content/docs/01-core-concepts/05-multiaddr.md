---
title: Multiaddr
description: 多地址格式——协议无关的寻址方案
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

回顾第三章的 Ping 示例，我们用这样的地址连接节点：

```text
/ip4/127.0.0.1/tcp/54321
```

这种 `/协议/值/协议/值` 的格式叫 **Multiaddr**（多地址）。与传统的 `IP:端口` 不同，它是一种自描述、可组合的地址格式——同一个节点可能通过 TCP、QUIC、WebSocket 等不同协议可达，Multiaddr 能统一表达这些"通往节点的不同道路"。

:::tip[配套工具]
本教程配套的桌面应用提供了 **Multiaddr 解析器**，可视化展示协议栈分层结构。侧边栏选择「Multiaddr 解析」即可使用。
:::

## 组件与组合

Multiaddr 由多个组件组成，每个组件包含协议名和参数值。组件可以像乐高积木一样拼接：

```text
/ip4/198.51.100.0/tcp/80/ws
```

从左到右描述了协议栈：IPv4 地址 → TCP 端口 80 → WebSocket。

### 常见组件

| 组件 | 说明 | 示例 |
| ---- | ---- | ---- |
| `ip4` / `ip6` | IP 地址 | `/ip4/192.168.1.1` |
| `dns` / `dns4` / `dns6` | DNS 域名 | `/dns4/example.com` |
| `dnsaddr` | 从 TXT 记录解析完整地址 | `/dnsaddr/bootstrap.libp2p.io` |
| `tcp` / `udp` | 传输层端口 | `/tcp/8080` |
| `quic-v1` | QUIC 协议 | `/udp/9000/quic-v1` |
| `ws` / `wss` | WebSocket | `/tcp/443/wss` |
| `p2p` | 节点 PeerId | `/p2p/12D3KooW...` |
| `p2p-circuit` | 中继连接 | `/p2p-circuit/p2p/12D3KooW...` |

### 实际示例

**本地 TCP 连接（带 PeerId）：**

```text
/ip4/127.0.0.1/tcp/54321/p2p/12D3KooWAbCd...
```

**生产环境 QUIC：**

```text
/ip4/147.75.87.27/udp/4001/quic-v1/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb
```

## p2p 组件：身份绑定

`p2p` 组件将传输地址与节点身份结合：

```text
/ip4/198.51.100.0/tcp/1234/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT
```

连接时，libp2p 会先用传输地址建立连接，再通过 Noise 握手验证对方 PeerId——确保连接到的是目标节点而非伪装者。

## DNS 解析

`dns4`/`dns6` 会解析域名的 A/AAAA 记录：

```text
/dns4/example.com/tcp/4001  →  /ip4/93.184.216.34/tcp/4001
```

`dnsaddr` 是 libp2p 专属方案，从 TXT 记录读取完整 Multiaddr，一个域名可解析出多个地址（不同协议、不同 IP 版本）：

```bash
dig +short _dnsaddr.bootstrap.libp2p.io txt
# 返回多条 dnsaddr=/ip4/.../tcp/.../p2p/... 记录
```

:::tip[生产建议]
引导节点优先使用 `dnsaddr`——支持多地址、无缝迁移、包含完整协议信息。
:::

## Rust API

```rust
use libp2p::{Multiaddr, multiaddr::Protocol};

// 解析
let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse()?;

// 遍历组件
for p in addr.iter() {
    match p {
        Protocol::Ip4(ip) => println!("IPv4: {ip}"),
        Protocol::Tcp(port) => println!("TCP: {port}"),
        Protocol::P2p(id) => println!("PeerId: {id}"),
        _ => {}
    }
}

// 提取 PeerId
let peer_id = addr.iter().find_map(|p| match p {
    Protocol::P2p(id) => Some(id),
    _ => None,
});
```

## 中继地址

NAT 后的节点可通过中继转发，使用 `p2p-circuit` 组件：

```text
/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7Pf.../p2p-circuit/p2p/QmVT6GYw...
```

含义：先连中继节点 `QmdPU7Pf...`，再请求转发到目标 `QmVT6GYw...`。后续章节详细讨论。

## 小结

Multiaddr 是 libp2p 的协议无关寻址方案：

- **自描述**：地址包含完整协议栈信息
- **可组合**：按需拼接网络路径
- **身份绑定**：`p2p` 组件关联传输地址与节点身份

下一章学习 Swarm——libp2p 如何管理这些连接。
