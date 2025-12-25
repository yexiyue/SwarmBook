---
title: Multiaddr
description: 多地址格式——协议无关的寻址方案
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

## 地址的困境

上一章我们解决了"身份"问题——PeerId 让每个节点拥有一个全局唯一、可验证的标识。但身份只是故事的一半。知道某人是谁，不代表你能找到他。

在传统网络中，找到某个服务很简单：`192.168.1.100:8080` 或 `example.com:443`。IP 地址加端口号，全世界通用。但这种简洁背后，隐藏着一个假设：**所有人都使用 TCP/IP**。

这个假设在 90 年代或许成立，但今天的网络世界早已不同：

- **传输协议多样化**：TCP、UDP、QUIC、WebSocket、WebRTC……每种协议都有自己的寻址方式
- **网络环境复杂化**：IPv4、IPv6、DNS、NAT、中继服务器……节点可能通过多种路径可达
- **安全需求差异化**：有的连接需要 TLS，有的需要 Noise，有的运行在 Tor 之上

如果你要设计一个跨平台、跨协议的 P2P 框架，第一个问题就是：**用什么格式来表示"怎么找到某个节点"？**

传统方案会把这些信息塞进一个复杂的配置对象：

```json
{
  "host": "192.168.1.100",
  "port": 8080,
  "protocol": "tcp",
  "encryption": "tls",
  "peer_id": "12D3KooW..."
}
```

但这种方式缺乏统一标准——每个项目都有自己的格式，互操作性为零。更糟的是，它无法表达复杂的网络路径：通过 DNS 解析、经过中继节点、使用特定加密协议……

libp2p 的解决方案是 **Multiaddr**——一种自描述、可组合、协议无关的地址格式。

## Multiaddr：地址的乐高积木

Multiaddr（多地址）的核心思想很简单：**把网络协议栈编码成一个可读的字符串**。

一个典型的 Multiaddr 看起来像这样：

```text
/ip4/192.168.1.100/tcp/8080
```

它由多个"组件"组成，每个组件包含一个协议名和一个参数值，用 `/` 分隔。这个地址表示：使用 IPv4 地址 `192.168.1.100`，通过 TCP 协议的 `8080` 端口连接。

与传统的 `192.168.1.100:8080` 相比，Multiaddr 的优势在于**自描述性**——地址本身就告诉你应该用什么协议。

### 组件与组合

Multiaddr 的威力在于组合。每个组件是一块乐高积木，你可以按需拼接：

```text
/ip4/198.51.100.0/tcp/80/ws
```

这个地址从外到内描述了完整的协议栈：

1. `ip4/198.51.100.0` — 网络层，IPv4 地址
2. `tcp/80` — 传输层，TCP 端口 80
3. `ws` — 应用层，WebSocket 协议

就像俄罗斯套娃，外层协议"封装"内层协议。数据从 WebSocket 发出，被 TCP 封装成数据段，再被 IP 封装成数据包，最终在网络中传输。Multiaddr 的顺序正好反映了这种封装关系。

### 常见组件

libp2p 支持丰富的协议组件：

| 组件 | 说明 | 示例 |
| ---- | ---- | ---- |
| `ip4` | IPv4 地址 | `/ip4/192.168.1.1` |
| `ip6` | IPv6 地址 | `/ip6/::1` |
| `dns` | DNS 域名（解析为 IPv4/IPv6） | `/dns/example.com` |
| `dns4` | DNS 域名（仅 IPv4） | `/dns4/example.com` |
| `dns6` | DNS 域名（仅 IPv6） | `/dns6/example.com` |
| `dnsaddr` | 从 TXT 记录解析完整地址 | `/dnsaddr/bootstrap.libp2p.io` |
| `tcp` | TCP 端口 | `/tcp/8080` |
| `udp` | UDP 端口 | `/udp/9000` |
| `quic-v1` | QUIC 协议 | `/udp/9000/quic-v1` |
| `ws` | WebSocket | `/tcp/80/ws` |
| `wss` | WebSocket over TLS | `/tcp/443/wss` |
| `p2p` | 节点 PeerId | `/p2p/12D3KooW...` |
| `p2p-circuit` | 中继连接 | `/p2p-circuit/p2p/12D3KooW...` |

### 完整地址示例

让我们看几个实际场景中的 Multiaddr：

#### 本地开发环境

```text
/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT
```

本地 TCP 连接到某个节点。

#### 生产环境（QUIC）

```text
/ip4/147.75.87.27/udp/4001/quic-v1/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb
```

使用 QUIC 协议连接，比 TCP 更快、更安全。

#### 浏览器环境（WebSocket）

```text
/dns4/example.com/tcp/443/wss/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT
```

浏览器通过 WebSocket over TLS 连接。

#### 中继连接

```text
/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7Pf.../p2p-circuit/p2p/QmVT6GYw...
```

当两个节点无法直连时，通过第三方中继节点转发。

## p2p 组件：身份与位置的结合

Multiaddr 有一个特殊组件 `p2p`，它的参数是节点的 PeerId：

```text
/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT
```

单独的 `p2p` 地址无法定位节点——它只是身份标识，不包含网络位置。但它可以**封装**在传输地址之后：

```text
/ip4/198.51.100.0/tcp/1234/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT
```

这种"传输地址 + PeerId"的组合是 libp2p 节点交换地址的标准格式。当你连接到这个地址时，libp2p 会：

1. 使用传输地址 `/ip4/198.51.100.0/tcp/1234` 建立连接
2. 在加密握手中验证对方的 PeerId 是否匹配 `12D3KooW...`

这确保了你连接到的确实是目标节点，而不是伪装者。

:::note[历史注记]
`p2p` 协议组件最初叫 `ipfs`，在某些旧代码中你可能会看到 `/ipfs/Qm...` 格式。两者在二进制层面完全等价，但现代 libp2p 已统一使用 `p2p`。
:::

## DNS 与名称解析

硬编码 IP 地址不利于维护——服务器迁移后地址就失效了。Multiaddr 支持多种基于 DNS 的解析方式：

### dns / dns4 / dns6

```text
/dns4/example.com/tcp/4001
```

libp2p 会解析 `example.com` 的 A 记录，得到 IPv4 地址，然后替换成：

```text
/ip4/93.184.216.34/tcp/4001
```

使用 `dns6` 则解析 AAAA 记录得到 IPv6 地址；`dns` 同时解析两者，优先使用 IPv6。

### dnsaddr：libp2p 专属的 DNS 方案

`dnsaddr` 是 libp2p 特有的解析方式，它从 DNS TXT 记录中读取完整的 Multiaddr。

例如，解析 `/dnsaddr/bootstrap.libp2p.io` 时，libp2p 会查询 `_dnsaddr.bootstrap.libp2p.io` 的 TXT 记录：

```bash
dig +short _dnsaddr.am6.bootstrap.libp2p.io txt
```

返回结果类似：

```text
"dnsaddr=/ip4/147.75.87.27/tcp/4001/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip4/147.75.87.27/udp/4001/quic-v1/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
"dnsaddr=/ip6/2604:1380:4602:5c00::3/tcp/4001/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb"
```

一个 `dnsaddr` 可以解析出多个地址，包含不同的传输协议（TCP、QUIC）和 IP 版本。这让运维人员可以灵活管理引导节点，而客户端只需记住一个域名。

:::tip[实践建议]
在生产环境中，优先使用 `dnsaddr` 发布引导节点地址。它比直接发布 IP 地址更灵活：

- 可以同时发布多个备用地址
- 支持无缝迁移（修改 DNS 记录即可）
- 包含完整的传输协议信息和 PeerId

:::

## 封装与解封装

Multiaddr 的核心操作是**封装（encapsulation）**和**解封装（decapsulation）**。

### 封装

把一个 Multiaddr "包"进另一个：

```rust
use libp2p::Multiaddr;

let ip: Multiaddr = "/ip4/192.168.1.1".parse().unwrap();
let tcp: Multiaddr = "/tcp/8080".parse().unwrap();

// 封装：ip + tcp
let full = ip.with(tcp.iter().next().unwrap());
// 结果：/ip4/192.168.1.1/tcp/8080
```

### 解封装

从组合地址中移除某个组件：

```rust
use libp2p::Multiaddr;

let addr: Multiaddr = "/ip4/192.168.1.1/tcp/8080/ws".parse().unwrap();

// 解封装：移除 /ws
let without_ws = addr.with(|p| p.filter(|c| c != Protocol::Ws));
// 结果：/ip4/192.168.1.1/tcp/8080
```

解封装会移除**最后出现**的匹配组件以及它封装的所有内层协议。例如，从 `/ip4/1.2.3.4/tcp/80/ws` 解封装 `/tcp/80`，结果是 `/ip4/1.2.3.4`，而不是 `/ip4/1.2.3.4/ws`——因为单独的 `/ip4/.../ws` 没有意义。

## Rust 中的 Multiaddr

rust-libp2p 提供了 `Multiaddr` 类型，用于解析、操作和构建多地址。

### 解析与创建

```rust
use libp2p::Multiaddr;
use std::str::FromStr;

// 从字符串解析
let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080".parse().unwrap();
// 或者
let addr = Multiaddr::from_str("/ip4/127.0.0.1/tcp/8080").unwrap();

// 程序化构建
use libp2p::multiaddr::Protocol;

let mut addr = Multiaddr::empty();
addr.push(Protocol::Ip4([127, 0, 0, 1].into()));
addr.push(Protocol::Tcp(8080));
```

### 遍历组件

```rust
use libp2p::{Multiaddr, multiaddr::Protocol};

let addr: Multiaddr = "/ip4/192.168.1.1/tcp/8080/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT".parse().unwrap();

for protocol in addr.iter() {
    match protocol {
        Protocol::Ip4(ip) => println!("IPv4: {ip}"),
        Protocol::Tcp(port) => println!("TCP port: {port}"),
        Protocol::P2p(peer_id) => println!("PeerId: {peer_id}"),
        _ => {}
    }
}
```

### 提取 PeerId

一个常见操作是从完整地址中提取 PeerId：

```rust
use libp2p::{Multiaddr, PeerId, multiaddr::Protocol};

fn extract_peer_id(addr: &Multiaddr) -> Option<PeerId> {
    addr.iter().find_map(|p| match p {
        Protocol::P2p(peer_id) => Some(peer_id),
        _ => None,
    })
}

let addr: Multiaddr = "/ip4/127.0.0.1/tcp/8080/p2p/12D3KooWMgcJeCtWqiptgnn4HHY6jRkMaRzHumXrntP3Np7qUqMT".parse().unwrap();
if let Some(peer_id) = extract_peer_id(&addr) {
    println!("Connecting to peer: {peer_id}");
}
```

### 验证传输地址

不是所有 Multiaddr 都是有效的传输地址。`/ip4/1.2.3.4` 缺少端口，`/tcp/8080` 缺少 IP：

```rust
use libp2p::Multiaddr;

fn is_valid_tcp_addr(addr: &Multiaddr) -> bool {
    let protocols: Vec<_> = addr.iter().collect();
    matches!(
        protocols.as_slice(),
        [Protocol::Ip4(_) | Protocol::Ip6(_) | Protocol::Dns(_) | Protocol::Dns4(_) | Protocol::Dns6(_),
         Protocol::Tcp(_), ..]
    )
}
```

## 二进制编码

Multiaddr 不仅有人类可读的字符串格式，还有紧凑的二进制格式，用于网络传输和存储。

每个协议组件编码为：

```
<protocol-code><address-bytes>
```

- `protocol-code` 是一个 [unsigned varint](https://github.com/multiformats/unsigned-varint)，标识协议类型
- `address-bytes` 是协议特定的地址数据（长度由协议定义）

例如，`/ip4/127.0.0.1/tcp/8080` 的二进制编码是：

```text
04          # ip4 协议码
7f 00 00 01 # 127.0.0.1 (4 字节)
06          # tcp 协议码
1f 90       # 8080 (2 字节，大端序)
```

完整的协议码表见 [multiaddr protocol table](https://github.com/multiformats/multiaddr/blob/master/protocols.csv)。

:::note[实现互操作性]
如果你需要与其他语言的 libp2p 实现交换地址，确保使用 `Multiaddr::to_vec()` 获取标准二进制格式，而不是自己序列化字符串。
:::

## 中继地址：穿越防火墙

当两个节点都在 NAT 后面无法直连时，可以通过第三方**中继节点**转发流量。中继地址使用 `p2p-circuit` 组件：

```text
/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7Pf.../p2p-circuit/p2p/QmVT6GYw...
```

这个地址表示：

1. 先连接到中继节点 `/ip4/192.0.2.0/tcp/5002/p2p/QmdPU7Pf...`
2. 通过中继请求连接目标节点 `/p2p/QmVT6GYw...`

中继连接是 libp2p 解决 NAT 穿透问题的重要手段，我们会在后续章节详细讨论。

## 小结

本章介绍了 Multiaddr——libp2p 的协议无关寻址方案：

- **自描述**：地址本身包含完整的协议栈信息
- **可组合**：像乐高积木一样，按需组装网络路径
- **跨平台**：支持 TCP、UDP、QUIC、WebSocket 等多种传输
- **身份绑定**：`p2p` 组件将传输地址与节点身份结合

回到开篇的问题——如何在一个多协议、多平台的世界里表达"怎么找到某个节点"？Multiaddr 的答案是：**用一种统一的、可扩展的格式，让地址自己说话**。

有了 PeerId 和 Multiaddr，我们已经具备了 libp2p 的两个基础概念：身份和地址。下一章，我们将学习 Swarm——libp2p 如何管理所有这些连接。
