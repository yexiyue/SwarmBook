# SwarmBook

和我一起探索去中心化网络的奥秘 - Rust libp2p 学习笔记与实战

## 项目结构

```
swarmbook/
├── docs/              # Astro + Starlight 教程站点
├── 01-ping/           # 章节示例代码
├── 02-xxx/
├── ...
├── Cargo.toml         # Rust workspace
└── README.md
```

## 教程目录

### 第一篇：P2P 基础概念

- 什么是 P2P
- libp2p 概览
- 节点身份
- Multiaddr

### 第二篇：传输层与安全

- 传输层抽象
- TCP 与 QUIC
- WebSocket
- Noise 协议
- 流复用
- 连接升级

### 第三篇：Swarm 与协议

- Swarm
- NetworkBehaviour
- Ping 协议
- 请求-响应模式
- 流管理
- 自定义协议

### 第四篇：节点发现与路由

- mDNS
- Kademlia DHT
- Identify 协议
- Bootstrap

### 第五篇：消息传播

- GossipSub
- 主题网格
- 消息验证

### 第六篇：实战 - P2P yjs 后端

- yjs 基础
- 架构设计
- 同步协议
- Awareness
- 持久化
- 生产化

## 本地开发

```bash
# 运行示例
cargo run -p 01-ping

# 启动文档站点
cd docs
pnpm dev
```

## License

MIT
