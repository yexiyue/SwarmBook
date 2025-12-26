# Swarmbook 教程设计文档

## 定位

- **目标读者**：熟悉 Rust，想学习 P2P 网络编程的开发者
- **教程风格**：实践驱动，先跑起来再理解原理（"先会用，再懂为什么"）
- **最终目标**：构建一个基于 libp2p 的 yjs P2P 后端，实现去中心化协作编辑
- **章节策略**：基础概念小章节，实战项目大章节
- **规范参考**：[libp2p/specs](https://github.com/libp2p/specs) 官方规范解读
- **实践素材**：[rust-libp2p/examples](https://github.com/libp2p/rust-libp2p/tree/master/examples) 官方示例

---

## 章节规划

### 第一篇：libp2p 核心概念

> 目标：运行第一个 P2P 程序，再回头理解背后的核心抽象

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 01-what-is-p2p | P2P vs C/S 架构，去中心化的意义 | - | - |
| 02-libp2p-overview | libp2p 模块化设计，核心抽象层 | - | - |
| 03-first-node | **第一个 libp2p 节点**（先跑起来！） | [ping](https://github.com/libp2p/specs/blob/master/ping/ping.md) | [ping](https://github.com/libp2p/rust-libp2p/tree/master/examples/ping) |
| 04-peer-identity | PeerId、密钥对、节点身份 | [peer-ids](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md) | - |
| 05-multiaddr | 多地址格式，协议无关的寻址 | [addressing](https://github.com/libp2p/specs/blob/master/addressing/README.md) | - |
| 06-swarm | Swarm 连接管理，事件驱动模型 | - | - |

### 第二篇：协议与实践

> 目标：通过实践掌握 NetworkBehaviour，能实现自定义协议

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 07-identify | Identify 协议，组合多个 Behaviour | [identify](https://github.com/libp2p/specs/blob/master/identify/README.md) | [identify](https://github.com/libp2p/rust-libp2p/tree/master/examples/identify) |
| 08-request-response | 请求-响应模式 | - | [file-sharing](https://github.com/libp2p/rust-libp2p/tree/master/examples/file-sharing) |
| 09-streams | 流管理与生命周期 | - | [stream](https://github.com/libp2p/rust-libp2p/tree/master/examples/stream) |
| 10-custom-protocol | 自定义协议实现 | - | 实现 echo 协议 |

### 第三篇：传输层原理

> 目标：理解连接建立的完整流程，能切换不同传输协议

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 11-transport | 传输层抽象，Transport trait | [connections](https://github.com/libp2p/specs/blob/master/connections/README.md) | - |
| 12-tcp-quic | TCP vs QUIC，连接建立与性能对比 | [quic](https://github.com/libp2p/specs/blob/master/quic/README.md) | 修改 ping 使用 QUIC |
| 13-security | Noise/TLS 加密握手 | [noise](https://github.com/libp2p/specs/blob/master/noise/README.md) | - |
| 14-muxing | 流复用原理，yamux | [yamux](https://github.com/libp2p/specs/blob/master/yamux/README.md) | - |
| 15-upgrade | 连接升级，协议协商 | [connections](https://github.com/libp2p/specs/blob/master/connections/README.md) | - |

### 第四篇：节点发现

> 目标：理解节点如何找到彼此，能构建可发现的 P2P 网络

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 16-mdns | 局域网发现（mDNS） | [mdns](https://github.com/libp2p/specs/blob/master/discovery/mdns.md) | 为 ping 添加 mDNS |
| 17-kademlia | DHT 原理与节点路由 | [kad-dht](https://github.com/libp2p/specs/blob/master/kad-dht/README.md) | [distributed-key-value-store](https://github.com/libp2p/rust-libp2p/tree/master/examples/distributed-key-value-store) |
| 18-bootstrap | 引导节点与网络加入 | - | [ipfs-kad](https://github.com/libp2p/rust-libp2p/tree/master/examples/ipfs-kad) |
| 19-rendezvous | Rendezvous 协议 | [rendezvous](https://github.com/libp2p/specs/blob/master/rendezvous/README.md) | [rendezvous](https://github.com/libp2p/rust-libp2p/tree/master/examples/rendezvous) |

### 第五篇：消息传播

> 目标：理解 Pub/Sub 模型，能实现群组通信

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 20-pubsub-intro | Pub/Sub 模型概述 | [pubsub](https://github.com/libp2p/specs/blob/master/pubsub/README.md) | - |
| 21-gossipsub | GossipSub 协议详解 | [gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md) | [chat](https://github.com/libp2p/rust-libp2p/tree/master/examples/chat) |
| 22-message-validation | 消息验证与签名 | [gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md) | 为 chat 添加消息验证 |

### 第六篇：生产环境

> 目标：理解 NAT 穿透与中继，能部署生产级 P2P 应用

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|------|
| 23-nat-problem | NAT 问题与解决方案 | - | - |
| 24-autonat | AutoNAT 自动检测 | [autonat](https://github.com/libp2p/specs/blob/master/autonat/README.md) | [autonat](https://github.com/libp2p/rust-libp2p/tree/master/examples/autonat) |
| 25-relay | 中继服务 | [relay](https://github.com/libp2p/specs/blob/master/relay/README.md) | [relay-server](https://github.com/libp2p/rust-libp2p/tree/master/examples/relay-server) |
| 26-dcutr | 直连升级（打洞） | [dcutr](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md) | [dcutr](https://github.com/libp2p/rust-libp2p/tree/master/examples/dcutr) |
| 27-metrics | 监控与可观测性 | - | [metrics](https://github.com/libp2p/rust-libp2p/tree/master/examples/metrics) |

### 第七篇：实战 - P2P 协作编辑

> 目标：综合运用所学，构建完整的 P2P 应用

| 章节 | 内容 | 实践 |
|-----|------|------|
| 28-yjs-intro | yjs 与 CRDT 原理 | - |
| 29-architecture | 系统架构设计 | - |
| 30-sync-protocol | 同步协议实现 | 基于 GossipSub |
| 31-awareness | 用户感知（光标、在线状态） | - |
| 32-persistence | 文档持久化 | - |
| 33-deployment | 部署与运维 | 完整项目 |

---

## 目录结构

```
docs/src/content/docs/
├── 01-core-concepts/        # 第一篇：libp2p 核心概念
│   ├── 01-what-is-p2p.md
│   ├── 02-libp2p-overview.md
│   ├── 03-first-node.md     # 先跑起来！
│   ├── 04-peer-identity.md
│   ├── 05-multiaddr.md
│   └── 06-swarm.md
│
├── 02-protocols/            # 第二篇：协议与实践
│   ├── 07-identify.md
│   ├── 08-request-response.md
│   ├── 09-streams.md
│   └── 10-custom-protocol.md
│
├── 03-transport/            # 第三篇：传输层原理
│   ├── 11-transport.md
│   ├── 12-tcp-quic.md
│   ├── 13-security.md
│   ├── 14-muxing.md
│   └── 15-upgrade.md
│
├── 04-discovery/            # 第四篇：节点发现
│   ├── 16-mdns.md
│   ├── 17-kademlia.md
│   ├── 18-bootstrap.md
│   └── 19-rendezvous.md
│
├── 05-pubsub/               # 第五篇：消息传播
│   ├── 20-pubsub-intro.md
│   ├── 21-gossipsub.md
│   └── 22-message-validation.md
│
├── 06-production/           # 第六篇：生产环境
│   ├── 23-nat-problem.md
│   ├── 24-autonat.md
│   ├── 25-relay.md
│   ├── 26-dcutr.md
│   └── 27-metrics.md
│
└── 07-yjs-collab/           # 第七篇：实战项目
    ├── 28-yjs-intro.md
    ├── 29-architecture.md
    ├── 30-sync-protocol.md
    ├── 31-awareness.md
    ├── 32-persistence.md
    └── 33-deployment.md
```

---

## 下一步

1. 重构现有目录结构以匹配新计划
2. 完成第一篇剩余章节
3. 逐篇推进，每完成一个知识点章节后运行对应 example
