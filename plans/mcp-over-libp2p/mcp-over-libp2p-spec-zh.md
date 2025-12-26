# MCP over libp2p 传输层规范

> 草案 v0.1.0 | 2025-12-25

## 摘要

本文档定义了 Model Context Protocol (MCP) 在 libp2p 流上的传输绑定。它使 MCP 服务端和客户端能够通过点对点网络直接通信，无需中心化基础设施。

## 状态

这是一份供社区讨论的草案规范，尚未成为 MCP 官方规范的一部分。

## 1. 引言

### 1.1 动机

现有的 MCP 传输方式（stdio、HTTP、SSE）要么需要本地进程通信，要么依赖中心化服务器。基于 libp2p 的传输层可以实现：

- **去中心化 MCP 服务**：无需中央服务器
- **NAT 穿透**：连接防火墙后的 MCP 服务
- **节点发现**：通过 DHT 发现 MCP 服务
- **端到端加密**：通过 libp2p Noise 协议内置支持

### 1.2 目标

- 定义在 libp2p 流上传输 MCP JSON-RPC 消息的标准方式
- 通过 Kademlia DHT 实现服务发现
- 保持与现有 MCP 协议语义的兼容性

### 1.3 非目标

- 修改 MCP 协议语义
- 定义新的 MCP 方法或能力
- 定义传输层身份之外的认证/授权机制

## 2. 协议标识符

```
/mcp/1.0.0
```

协议标识符遵循 libp2p 约定：
- `/mcp` - 协议名称（Model Context Protocol）
- `/1.0.0` - 与 MCP 规范版本匹配的版本号

## 3. 消息帧格式

MCP 使用 JSON-RPC 2.0 消息。在 libp2p 流上，消息使用长度前缀编码：

```
+------------------+------------------------+
| 长度 (4 字节)     | JSON-RPC 消息           |
| 大端序 u32       | UTF-8 编码              |
+------------------+------------------------+
```

### 3.1 帧格式

| 字段 | 大小 | 描述 |
|------|------|------|
| 长度 | 4 字节 | 消息长度（大端序无符号 32 位整数） |
| 载荷 | 可变 | UTF-8 编码的 JSON-RPC 2.0 消息 |

### 3.2 最大消息大小

实现必须（MUST）支持最大 16 MiB（16,777,216 字节）的消息。实现可以（MAY）拒绝超过此限制的消息。

### 3.3 示例

一个 `tools/list` 请求：

```json
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
```

编码为（十六进制）：
```
00 00 00 38                                     # 长度: 56 字节
7b 22 6a 73 6f 6e 72 70 63 22 3a 22 32 2e 30    # {"jsonrpc":"2.0"
22 2c 22 69 64 22 3a 31 2c 22 6d 65 74 68 6f    # ,"id":1,"metho
64 22 3a 22 74 6f 6f 6c 73 2f 6c 69 73 74 22    # d":"tools/list"
2c 22 70 61 72 61 6d 73 22 3a 7b 7d 7d          # ,"params":{}}
```

## 4. 连接生命周期

### 4.1 连接建立

1. 客户端使用协议 `/mcp/1.0.0` 向服务端打开 libp2p 流
2. 服务端接受该流
3. 双方现在可以交换 MCP 消息

### 4.2 初始化

流建立后，执行标准 MCP 初始化握手：

```
客户端                              服务端
   |                                   |
   |-------- initialize 请求 --------->|
   |<------- initialize 响应 ----------|
   |                                   |
   |--- initialized 通知 ------------->|
   |                                   |
```

### 4.3 消息交换

初始化后，消息双向流动：
- 客户端发送请求，服务端发送响应
- 任一方都可以发送通知
- 可以有多个请求同时进行（通过 `id` 匹配）

### 4.4 连接终止

任一方都可以随时关闭流。实现应该（SHOULD）：
- 尽可能在关闭前完成进行中的请求
- 优雅地处理突然断开的情况

## 5. 服务发现

### 5.1 DHT Provider 记录

MCP 服务端可以（MAY）使用 provider 记录向 Kademlia DHT 公告自己。

**Key 派生：**
```
key = SHA256("mcp-service:" + service_name)
```

**Provider 记录内容：**
```json
{
  "name": "my-knowledge-base",
  "version": "1.0.0",
  "capabilities": ["tools", "resources"],
  "tools": ["search", "query"],
  "metadata": {
    "description": "一个知识库服务"
  }
}
```

### 5.2 发现流程

1. 客户端计算所需服务/能力的 key
2. 客户端向 DHT 查询该 key 的 providers
3. DHT 返回 PeerId 列表
4. 客户端连接到节点并打开 MCP 流

### 5.3 预定义 Key

| Key | 用途 |
|-----|------|
| `SHA256("mcp-service:*")` | 所有 MCP 服务 |
| `SHA256("mcp-capability:tools")` | 提供 tools 的服务 |
| `SHA256("mcp-capability:resources")` | 提供 resources 的服务 |
| `SHA256("mcp-capability:prompts")` | 提供 prompts 的服务 |

## 6. 安全考虑

### 6.1 传输安全

libp2p 通过 Noise 协议提供传输层安全。所有 MCP 消息在传输中都是加密的。

### 6.2 节点身份

每个节点都有一个加密身份（PeerId 由公钥派生）。实现应该（SHOULD）：
- 在交换敏感数据前验证节点身份
- 维护可信节点的白名单/黑名单

### 6.3 资源限制

实现必须（MUST）防止资源耗尽：
- 限制每个节点的并发流数量
- 限制消息大小（见第 3.2 节）
- 对请求实施速率限制

### 6.4 工具执行

MCP 工具执行的安全性不在本传输规范范围内。实现应该（SHOULD）遵循 MCP 工具沙箱的安全最佳实践。

## 7. 实现说明

### 7.1 推荐的 libp2p 配置

```
传输层: TCP + QUIC
安全: Noise
多路复用: Yamux
发现: Kademlia DHT + mDNS（本地）
```

### 7.2 Rust 实现

参考实现：`rmcp-transport-p2p` crate

```rust
use rmcp::ServiceExt;
use rmcp_transport_p2p::P2pTransport;

// 服务端
my_service().serve(P2pTransport::new(config)).await?;

// 客户端
let client = ().serve(P2pTransport::connect(peer_id, config)).await?;
```

## 8. 参考资料

- [MCP 规范](https://spec.modelcontextprotocol.io/)
- [libp2p 规范](https://github.com/libp2p/specs)
- [JSON-RPC 2.0](https://www.jsonrpc.org/specification)
- [Kademlia DHT](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)

## 附录 A：协议协商

当节点支持多个 MCP 版本时，协议协商遵循 libp2p multistream-select：

```
/mcp/1.0.0
/mcp/0.9.0  （假设的旧版本）
```

节点协商双方都支持的最高版本。

## 附录 B：与其他传输方式对比

| 传输方式 | 服务发现 | NAT 穿透 | 加密 | 去中心化 |
|----------|----------|----------|------|----------|
| stdio | 不适用 | 不适用 | 不适用 | 不适用 |
| HTTP | 手动 | 否 | TLS（可选） | 否 |
| SSE | 手动 | 否 | TLS（可选） | 否 |
| **libp2p** | DHT | 是 | Noise | 是 |

## 更新日志

- v0.1.0 (2025-12-25): 初始草案
