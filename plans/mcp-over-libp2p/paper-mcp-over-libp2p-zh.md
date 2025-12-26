# MCP over libp2p：模型上下文协议的去中心化传输层

> 草稿 v0.1 | 2025年12月

## 摘要

模型上下文协议（Model Context Protocol，MCP）已成为大型语言模型（LLM）应用与外部工具和数据源交互的标准化接口。然而，现有的 MCP 传输机制（stdio、HTTP、SSE）要么依赖本地进程通信，要么依赖中心化服务器基础设施，限制了其在去中心化场景中的适用性。本文提出 MCP over libp2p，一种新型传输层，使 MCP 服务端和客户端能够通过点对点网络直接通信。我们的方案利用 libp2p 强大的网络栈提供 NAT 穿透、端到端加密以及基于 Kademlia DHT 的去中心化服务发现。我们定义了协议规范，提供了 Rust 参考实现，并评估了其性能特征。实验结果表明，MCP over libp2p 在实现与 HTTP 传输相当的延迟的同时，能够支持联邦知识网络和去中心化 AI Agent 协作等新型应用场景。

**关键词**：模型上下文协议、libp2p、点对点网络、去中心化AI、Agent通信

---

## 1. 引言

### 1.1 背景

大型语言模型（LLM）的快速发展催生了能够使用外部工具和数据源的 AI Agent。Anthropic 于 2024 年推出的模型上下文协议（MCP）为 LLM 应用与工具、资源和提示词的交互提供了基于 JSON-RPC 2.0 的标准化接口 [1]。截至 2025 年，MCP 已获得广泛采用，每周 SDK 下载量超过 800 万次 [2]。

目前 MCP 实现支持三种传输机制：
- **stdio**：通过标准输入/输出的本地进程通信
- **HTTP**：传统的客户端-服务器通信
- **SSE（服务器发送事件）**：基于 HTTP 的流式传输

虽然这些传输方式能够很好地服务于中心化部署场景，但对于新兴的去中心化 AI 应用存在以下局限：

1. **单点故障**：基于 HTTP 的传输需要中心化服务器
2. **NAT 穿透**：防火墙后的客户端难以暴露 MCP 服务
3. **服务发现**：缺乏发现可用 MCP 服务的标准化机制
4. **隐私性**：所有通信都需要通过中心化基础设施路由

### 1.2 动机

去中心化 AI 系统日益增长的需求推动了对点对点 MCP 传输的需求。具体应用场景包括：

- **联邦知识网络**：多个节点在无中心协调的情况下共享知识库
- **分布式 AI Agent**：Agent 跨组织边界进行协作
- **隐私保护型 AI**：无中间人的直接点对点通信
- **弹性 AI 基础设施**：无单点故障

### 1.3 贡献

本文的主要贡献如下：

1. **协议规范**：我们定义了 MCP over libp2p，一种基于 libp2p 流的 MCP 传输绑定（第 3 节）
2. **服务发现**：我们提出了基于 DHT 的 MCP 服务发现机制（第 4 节）
3. **参考实现**：我们提供了 Rust 实现 `rmcp-transport-p2p`（第 5 节）
4. **性能评估**：我们评估了性能特征并与现有传输进行了对比（第 6 节）

---

## 2. 相关工作

### 2.1 模型上下文协议

MCP 由 Anthropic 于 2024 年底作为 LLM-工具集成的开放协议推出 [1]。该协议定义了三个核心原语：
- **工具（Tools）**：LLM 可以调用的可执行函数
- **资源（Resources）**：LLM 可以读取的数据源
- **提示词（Prompts）**：可复用的提示词模板

近期学术研究分析了 MCP 的架构和安全特性 [2][3]，但传输层扩展尚未被探索。

### 2.2 Agent 通信协议

多个协议致力于解决 Agent 间通信问题：

**Agent 网络协议（ANP）**[4] 使用 W3C DIDs 和 JSON-LD 提供去中心化 Agent 协作。ANP 以 P2P 配置使用 HTTP，但缺乏原生 P2P 网络能力。

**Agent 到 Agent 协议（A2A）**[5] 由 Google 提出，专注于 Agent 互操作性，但依赖传统 HTTP 传输。

**DIAP** [6] 将 libp2p 与零知识证明结合用于 Agent 身份验证，证明了 libp2p 用于 Agent 系统的可行性。

### 2.3 libp2p

libp2p 是最初为 IPFS 开发的模块化点对点网络栈 [7]。主要特性包括：
- 多种传输协议（TCP、QUIC、WebSocket）
- 通过 Noise 协议内置加密
- NAT 穿透技术
- 用于节点发现的 Kademlia DHT
- 协议多路复用

libp2p 已成功部署于 IPFS、Ethereum 2.0 和 Filecoin 等生产系统。

---

## 3. 协议设计

### 3.1 协议标识符

我们遵循 libp2p 约定定义协议标识符：

```
/mcp/1.0.0
```

版本号与 MCP 规范版本对齐以确保兼容性。

### 3.2 消息帧格式

MCP 消息采用 JSON-RPC 2.0 格式。在 libp2p 流上，我们使用长度前缀帧格式：

```
+------------------+------------------------+
| 长度 (4 字节)     | JSON-RPC 消息           |
| 大端序 u32       | UTF-8 编码              |
+------------------+------------------------+
```

此帧格式的特点：
- 支持高效消息解析
- 支持最大 4GB 的消息（实际限制：16MB）
- 保持与现有 MCP 消息格式的兼容性

### 3.3 连接生命周期

```
┌────────┐                              ┌────────┐
│ 客户端  │                              │ 服务端  │
└───┬────┘                              └───┬────┘
    │                                       │
    │  1. libp2p 拨号 (协议: /mcp/1.0.0)     │
    │──────────────────────────────────────>│
    │                                       │
    │  2. 流建立                             │
    │<──────────────────────────────────────│
    │                                       │
    │  3. MCP initialize 请求               │
    │──────────────────────────────────────>│
    │                                       │
    │  4. MCP initialize 响应               │
    │<──────────────────────────────────────│
    │                                       │
    │  5. MCP initialized 通知              │
    │──────────────────────────────────────>│
    │                                       │
    │  6. 正常 MCP 消息交换                   │
    │<─────────────────────────────────────>│
    │                                       │
```

libp2p 流建立后，按照 MCP 规范定义执行标准 MCP 初始化握手。

### 3.4 安全模型

libp2p 通过 Noise 协议框架提供传输层安全：
- 所有连接都是加密的
- 通过 PeerId 进行密码学验证的节点身份
- 防止中间人攻击

应用层安全（授权、访问控制）仍由 MCP 服务实现负责。

---

## 4. 服务发现

### 4.1 基于 DHT 的发现

我们利用 Kademlia DHT 进行去中心化服务发现。MCP 服务端将自己作为特定键的提供者进行公告。

**键派生**：
```
key = SHA256("mcp-service:" + service_name)
```

**Provider 记录**：
```json
{
  "name": "knowledge-base",
  "version": "1.0.0",
  "capabilities": ["tools", "resources"],
  "tools": ["search", "query"],
  "metadata": {}
}
```

### 4.2 基于能力的发现

我们定义了用于基于能力发现的预定义键：

| 键 | 用途 |
|-----|---------|
| `SHA256("mcp-service:*")` | 所有 MCP 服务 |
| `SHA256("mcp-capability:tools")` | 提供工具的服务 |
| `SHA256("mcp-capability:resources")` | 提供资源的服务 |
| `SHA256("mcp-capability:prompts")` | 提供提示词的服务 |

### 4.3 发现流程

```
┌────────┐         ┌─────┐         ┌────────┐
│ 客户端  │         │ DHT │         │ 服务端  │
└───┬────┘         └──┬──┘         └───┬────┘
    │                 │                │
    │  1. 注册        │                │
    │                 │<───────────────│
    │                 │                │
    │  2. 查询        │                │
    │────────────────>│                │
    │                 │                │
    │  3. PeerIds     │                │
    │<────────────────│                │
    │                 │                │
    │  4. 通过 /mcp/1.0.0 连接           │
    │─────────────────────────────────>│
    │                 │                │
```

---

## 5. 实现

### 5.1 架构

我们将 `rmcp-transport-p2p` 实现为与官方 `rmcp` SDK 集成的 Rust crate：

```
┌─────────────────────────────────────┐
│         rmcp-transport-p2p          │
├─────────────┬───────────────────────┤
│ P2pTransport│    ServiceRegistry    │
├─────────────┼───────────────────────┤
│  McpCodec   │     McpProtocol       │
└─────────────┴───────────────────────┘
              │
┌─────────────┴───────────────────────┐
│            libp2p                   │
│  (Swarm, Kademlia, Noise, Yamux)    │
└─────────────────────────────────────┘
```

### 5.2 API 设计

**服务端**：
```rust
use rmcp::ServiceExt;
use rmcp_transport_p2p::P2pTransport;

my_mcp_service()
    .serve(P2pTransport::new(config))
    .await?;
```

**客户端**：
```rust
use rmcp::ClientExt;
use rmcp_transport_p2p::P2pTransport;

let client = ()
    .serve(P2pTransport::connect(peer_id, config))
    .await?;
```

API 与现有 rmcp 传输用法保持一致，便于无缝采用。

### 5.3 实现细节

关键实现考虑：

1. **流管理**：每个 MCP 会话使用专用的 libp2p 流
2. **连接池**：复用连接以减少与同一节点重复交互的握手开销
3. **背压**：通过 libp2p 内置机制进行流量控制
4. **错误处理**：将 libp2p 错误映射为 MCP 错误响应

---

## 6. 评估

### 6.1 实验设置

- **硬件**：2x Intel i7-12700，32GB 内存
- **网络**：本地网络（1Gbps）和模拟广域网（100ms RTT）
- **实现**：rmcp-transport-p2p v0.1.0，rust-libp2p 0.56

### 6.2 延迟对比

| 传输方式 | 本地 (ms) | 广域网 (ms) |
|-----------|------------|----------|
| stdio | 0.5 | 不适用 |
| HTTP | 2.1 | 105.3 |
| libp2p (TCP) | 3.2 | 108.7 |
| libp2p (QUIC) | 2.8 | 106.2 |

libp2p 传输相比 HTTP 仅增加约 1ms 的额外开销。

### 6.3 连接建立

| 场景 | 时间 (ms) |
|----------|-----------|
| 直连 | 15.3 |
| NAT 穿透（中继） | 89.7 |
| DHT 发现 + 连接 | 234.5 |

### 6.4 吞吐量

对于大型工具响应（1MB 负载）：

| 传输方式 | 吞吐量 (MB/s) |
|-----------|-------------------|
| HTTP | 112.4 |
| libp2p (TCP) | 98.7 |
| libp2p (QUIC) | 105.2 |

---

## 7. 讨论

### 7.1 优势

1. **去中心化**：无需中央服务器
2. **NAT 穿透**：内置打洞和中继支持
3. **安全性**：默认端到端加密
4. **发现**：基于 DHT 的服务发现

### 7.2 局限性

1. **引导**：需要初始引导节点
2. **复杂性**：对于简单部署比 HTTP 更复杂
3. **移动端**：持久连接的电池消耗较高

### 7.3 未来工作

1. **与 ANP 集成**：结合 Agent 网络协议以获得更丰富的 Agent 能力
2. **身份**：集成 W3C DIDs 实现跨协议身份
3. **激励**：为服务提供者设计基于代币的激励机制

---

## 8. 结论

我们提出了 MCP over libp2p，一种用于模型上下文协议的去中心化传输层。我们的方案支持具有内置加密、NAT 穿透和服务发现功能的点对点 MCP 通信。参考实现表明，与 HTTP 传输相比，性能开销最小，具有实际可行性。

随着 AI 系统日益分布式化和基于 Agent 化，像 MCP over libp2p 这样的去中心化协议将在构建安全、弹性和隐私保护的 AI 基础设施中发挥关键作用。

---

## 参考文献

[1] Anthropic, "Model Context Protocol Specification," 2024. https://spec.modelcontextprotocol.io/

[2] H. Zhang et al., "Model Context Protocol (MCP): Landscape, Security Threats, and Future Research Directions," arXiv:2503.23278, 2025.

[3] L. Wang et al., "Model Context Protocol (MCP) at First Glance: Studying the Security and Maintainability of MCP Servers," arXiv:2506.13538, 2025.

[4] Agent Network Protocol, "ANP Technical White Paper," arXiv:2508.00007, 2025.

[5] Google, "Agent-to-Agent Protocol Specification," 2025.

[6] M. Chen et al., "DIAP: A Decentralized Agent Identity Protocol with Zero-Knowledge Proofs and a Hybrid P2P Stack," arXiv:2511.11619, 2025.

[7] J. Benet, "IPFS - Content Addressed, Versioned, P2P File System," arXiv:1407.3561, 2014.

[8] libp2p, "libp2p Specification," https://github.com/libp2p/specs

---

## 附录 A：协议规范

完整规范请参见：https://github.com/anthropics/mcp-over-libp2p-spec

## 附录 B：基准测试方法

所有基准测试使用 criterion.rs 进行，包含 100 次迭代和 10 次预热运行。广域网延迟注入通过 tc（流量控制）进行网络模拟。
