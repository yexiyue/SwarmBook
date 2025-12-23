# Swarmbook 教程设计文档

## 定位

- **目标读者**：熟悉 Rust，想学习 P2P 网络编程的开发者
- **教程风格**：原理 + 实践并重，每个概念先讲"为什么"，再讲"怎么做"
- **最终目标**：构建一个基于 libp2p 的 yjs P2P 后端，实现去中心化协作编辑
- **章节策略**：基础概念小章节，实战项目大章节
- **规范参考**：[libp2p/specs](https://github.com/libp2p/specs) 官方规范解读
- **实践素材**：[rust-libp2p/examples](https://github.com/libp2p/rust-libp2p/tree/master/examples) 官方示例

---

## 章节规划

### 第一篇：libp2p 核心概念

> 目标：理解 libp2p 的设计哲学和核心抽象，能运行第一个 P2P 程序

| 章节                 | 内容                          | 规范                                                                             | 实践                                                                      |
| ------------------ | --------------------------- | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------- |
| 01-what-is-p2p     | P2P vs C/S 架构，去中心化的意义       | -                                                                              | -                                                                       |
| 02-libp2p-overview | libp2p 模块化设计，核心抽象层          | -                                                                              | -                                                                       |
| 03-peer-identity   | PeerId、密钥对、节点身份             | [peer-ids](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md)   | -                                                                       |
| 04-multiaddr       | 多地址格式，协议无关的寻址               | [addressing](https://github.com/libp2p/specs/blob/master/addressing/README.md) | -                                                                       |
| 05-swarm           | Swarm 连接管理，事件驱动模型           | -                                                                              | -                                                                       |
| 06-behaviour       | NetworkBehaviour trait，协议组合 | -                                                                              | -                                                                       |
| 07-first-node      | 第一个 libp2p 节点               | [ping](https://github.com/libp2p/specs/blob/master/ping/ping.md)               | [ping](https://github.com/libp2p/rust-libp2p/tree/master/examples/ping) |

### 第二篇：传输层与连接

> 目标：理解连接建立的完整流程，能切换不同传输协议

| 章节           | 内容                    | 规范                                                                               | 实践              |
| ------------ | --------------------- | -------------------------------------------------------------------------------- | --------------- |
| 08-transport | 传输层抽象，Transport trait | [connections](https://github.com/libp2p/specs/blob/master/connections/README.md) | -               |
| 09-tcp-quic  | TCP vs QUIC，连接建立与性能对比 | [quic](https://github.com/libp2p/specs/blob/master/quic/README.md)               | 修改 ping 使用 QUIC |
| 10-security  | Noise/TLS 加密握手        | [noise](https://github.com/libp2p/specs/blob/master/noise/README.md)             | -               |
| 11-muxing    | 流复用原理，yamux           | [yamux](https://github.com/libp2p/specs/blob/master/yamux/README.md)             | -               |
| 12-upgrade   | 连接升级，协议协商             | [connections](https://github.com/libp2p/specs/blob/master/connections/README.md) | -               |

### 第三篇：协议与流

> 目标：理解 libp2p 协议模型，能实现自定义协议

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|-----|
| 13-ping-deep | Ping 协议深度解析 | [ping](https://github.com/libp2p/specs/blob/master/ping/ping.md) | ping 源码解读 |
| 14-identify | Identify 协议，节点信息交换 | [identify](https://github.com/libp2p/specs/blob/master/identify/README.md) | [identify](https://github.com/libp2p/rust-libp2p/tree/master/examples/identify) |
| 15-request-response | 请求-响应模式 | - | [file-sharing](https://github.com/libp2p/rust-libp2p/tree/master/examples/file-sharing) |
| 16-streams | 流管理与生命周期 | - | [stream](https://github.com/libp2p/rust-libp2p/tree/master/examples/stream) |
| 17-custom-protocol | 自定义协议实现 | - | 实现 echo 协议 |

### 第四篇：节点发现

> 目标：理解节点如何找到彼此，能构建可发现的 P2P 网络

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|-----|
| 18-mdns | 局域网发现（mDNS） | [mdns](https://github.com/libp2p/specs/blob/master/discovery/mdns.md) | 为 ping 添加 mDNS |
| 19-kademlia | DHT 原理与节点路由 | [kad-dht](https://github.com/libp2p/specs/blob/master/kad-dht/README.md) | [distributed-key-value-store](https://github.com/libp2p/rust-libp2p/tree/master/examples/distributed-key-value-store) |
| 20-bootstrap | 引导节点与网络加入 | - | [ipfs-kad](https://github.com/libp2p/rust-libp2p/tree/master/examples/ipfs-kad) |
| 21-rendezvous | Rendezvous 协议 | [rendezvous](https://github.com/libp2p/specs/blob/master/rendezvous/README.md) | [rendezvous](https://github.com/libp2p/rust-libp2p/tree/master/examples/rendezvous) |

### 第五篇：消息传播

> 目标：理解 Pub/Sub 模型，能实现群组通信

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|-----|
| 22-pubsub-intro | Pub/Sub 模型概述 | [pubsub](https://github.com/libp2p/specs/blob/master/pubsub/README.md) | - |
| 23-gossipsub | GossipSub 协议详解 | [gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md) | [chat](https://github.com/libp2p/rust-libp2p/tree/master/examples/chat) |
| 24-message-validation | 消息验证与签名 | [gossipsub](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md) | 为 chat 添加消息验证 |

### 第六篇：生产环境

> 目标：理解 NAT 穿透与中继，能部署生产级 P2P 应用

| 章节 | 内容 | 规范 | 实践 |
|-----|------|-----|-----|
| 25-nat-problem | NAT 问题与解决方案 | - | - |
| 26-autonat | AutoNAT 自动检测 | [autonat](https://github.com/libp2p/specs/blob/master/autonat/README.md) | [autonat](https://github.com/libp2p/rust-libp2p/tree/master/examples/autonat) |
| 27-relay | 中继服务 | [relay](https://github.com/libp2p/specs/blob/master/relay/README.md) | [relay-server](https://github.com/libp2p/rust-libp2p/tree/master/examples/relay-server) |
| 28-dcutr | 直连升级（打洞） | [dcutr](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md) | [dcutr](https://github.com/libp2p/rust-libp2p/tree/master/examples/dcutr) |
| 29-metrics | 监控与可观测性 | - | [metrics](https://github.com/libp2p/rust-libp2p/tree/master/examples/metrics) |

### 第七篇：实战 - P2P 协作编辑

> 目标：综合运用所学，构建完整的 P2P 应用

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 30-yjs-intro | yjs 与 CRDT 原理 | - |
| 31-architecture | 系统架构设计 | - |
| 32-sync-protocol | 同步协议实现 | 基于 GossipSub |
| 33-awareness | 用户感知（光标、在线状态） | - |
| 34-persistence | 文档持久化 | - |
| 35-deployment | 部署与运维 | 完整项目 |

---

## 目录结构

```
docs/src/content/docs/
├── 01-core-concepts/        # 第一篇：libp2p 核心概念
│   ├── 01-what-is-p2p.md
│   ├── 02-libp2p-overview.md
│   ├── 03-peer-identity.md
│   ├── 04-multiaddr.md
│   ├── 05-swarm.md
│   ├── 06-behaviour.md
│   └── 07-first-node.md
│
├── 02-transport/            # 第二篇：传输层与连接
│   ├── 08-transport.md
│   ├── 09-tcp-quic.md
│   ├── 10-security.md
│   ├── 11-muxing.md
│   └── 12-upgrade.md
│
├── 03-protocols/            # 第三篇：协议与流
│   ├── 13-ping-deep.md
│   ├── 14-identify.md
│   ├── 15-request-response.md
│   ├── 16-streams.md
│   └── 17-custom-protocol.md
│
├── 04-discovery/            # 第四篇：节点发现
│   ├── 18-mdns.md
│   ├── 19-kademlia.md
│   ├── 20-bootstrap.md
│   └── 21-rendezvous.md
│
├── 05-pubsub/               # 第五篇：消息传播
│   ├── 22-pubsub-intro.md
│   ├── 23-gossipsub.md
│   └── 24-message-validation.md
│
├── 06-production/           # 第六篇：生产环境
│   ├── 25-nat-problem.md
│   ├── 26-autonat.md
│   ├── 27-relay.md
│   ├── 28-dcutr.md
│   └── 29-metrics.md
│
└── 07-yjs-collab/           # 第七篇：实战项目
    ├── 30-yjs-intro.md
    ├── 31-architecture.md
    ├── 32-sync-protocol.md
    ├── 33-awareness.md
    ├── 34-persistence.md
    └── 35-deployment.md
```

---

## 下一步

1. 重构现有目录结构以匹配新计划
2. 完成第一篇剩余章节（03-07）
3. 逐篇推进，每完成一个知识点章节后运行对应 example
