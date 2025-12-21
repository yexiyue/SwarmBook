# Swarmbook 教程设计文档

## 定位

- **目标读者**：熟悉 Rust，想学习 P2P 网络编程的开发者
- **教程风格**：原理 + 实践并重，每个概念先讲"为什么"，再讲"怎么做"
- **最终目标**：构建一个基于 libp2p 的 yjs P2P 后端，实现去中心化协作编辑
- **章节策略**：基础概念小章节，实战项目大章节
- **实践素材**：基于 rust-libp2p 官方 examples

---

## 章节规划

### 第一篇：P2P 基础概念

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 01-what-is-p2p | P2P vs C/S 架构，去中心化的意义 | - |
| 02-libp2p-overview | libp2p 模块化设计，核心抽象 | - |
| 03-peer-identity | PeerId、密钥对、节点身份 | `ping` example |
| 04-multiaddr | 多地址格式，协议无关的寻址 | - |

### 第二篇：传输层与安全

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 05-transport-abstraction | 传输层抽象，Transport trait | - |
| 06-tcp-quic | TCP vs QUIC 对比，连接建立 | `ping` 切换传输层 |
| 07-websocket | WebSocket 传输，浏览器兼容 | - |
| 08-noise | Noise 协议，加密握手流程 | - |
| 09-muxing | 流复用原理，yamux/mplex | - |
| 10-connection-upgrade | 连接升级，协议协商（multistream-select） | - |

### 第三篇：Swarm 与协议

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 11-swarm | Swarm 是什么，事件驱动模型 | - |
| 12-behaviour | NetworkBehaviour trait，组合行为 | - |
| 13-ping-protocol | 最简协议分析 | `ping` 源码解读 |
| 14-request-response | 请求-响应模式 | `file-sharing` example |
| 15-streams | 流与子流管理 | - |
| 16-custom-protocol | 自定义协议实现，握手设计 | 实现 echo 协议 |

### 第四篇：节点发现与路由

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 17-mdns | 局域网节点发现 | `chat` example |
| 18-kademlia | DHT 原理，节点路由 | `distributed-key-value-store` |
| 19-identify | 节点信息交换协议 | - |
| 20-bootstrap | 引导节点，网络加入 | - |

### 第五篇：消息传播

| 章节 | 内容 | 实践 |
|-----|------|-----|
| 21-gossipsub | Pub/Sub 原理，gossip 协议 | `chat` example |
| 22-topic-mesh | 主题网格，消息传播路径 | - |
| 23-message-validation | 消息验证与过滤 | - |

### 第六篇：实战 - P2P yjs 后端

| 章节 | 内容 |
|-----|------|
| 24-yjs-basics | yjs/CRDT 原理简介 |
| 25-architecture | 系统架构设计：纯 P2P vs 混合模式对比 |
| 26-sync-protocol | 设计同步协议，基于 gossipsub |
| 27-awareness | 用户感知（光标、在线状态） |
| 28-persistence | 文档持久化策略 |
| 29-production | 生产化考量：NAT 穿透、中继、信令 |

---

## 项目结构

```
swarmbook/
├── docs/                    # Astro 教程网站
│   ├── src/content/docs/    # 教程 markdown
│   └── plans/               # 设计文档
│
├── 01-ping/                 # 章节示例代码
├── 02-xxx/
├── ...
│
├── Cargo.toml               # workspace root
└── README.md
```

---

## 下一步

1. 初始化 Cargo workspace
2. 从第一篇开始编写，每完成一个知识点章节后运行对应 example
3. 第六篇实战项目在前五篇完成后开始
