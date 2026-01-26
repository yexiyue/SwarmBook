//! 分布式计数器协议示例
//!
//! 演示如何使用 libp2p_stream 实现一个简单的分布式计数器：
//! - 每个节点维护本地计数器
//! - 通过同步取最大值达成最终一致性
//!
//! 运行方式：
//! ```bash
//! cargo run -p counter
//! ```

use std::collections::HashSet;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use futures::{AsyncReadExt, AsyncWriteExt, StreamExt};
use libp2p::{
    Multiaddr, PeerId, StreamProtocol, SwarmBuilder, noise, swarm::SwarmEvent, tcp, yamux,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// 协议标识符
const COUNTER_PROTOCOL: StreamProtocol = StreamProtocol::new("/counter/1.0.0");

/// 从主线程发送到事件循环的命令
enum Command {
    /// 连接到指定地址
    Dial(Multiaddr),
    /// 与指定节点同步计数器
    Sync(PeerId),
    /// 本地计数器 +1
    Increment,
}

#[tokio::main]
async fn main() -> Result<()> {
    let theme = ColorfulTheme::default();

    // ========== 1. 创建 Swarm ==========
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,      // Noise 协议加密
            yamux::Config::default,  // Yamux 多路复用
        )?
        .with_behaviour(|_| libp2p_stream::Behaviour::new())?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    // ========== 2. 获取 Control 并注册协议 ==========
    let mut control = swarm.behaviour().new_control();
    let mut incoming = control.accept(COUNTER_PROTOCOL)?;

    // ========== 3. 共享状态 ==========
    // 原子计数器，可在多个任务间安全共享
    let counter = Arc::new(AtomicU64::new(0));
    // 已连接节点列表
    let peers: Arc<RwLock<HashSet<PeerId>>> = Arc::new(RwLock::new(HashSet::new()));

    // ========== 4. 入站流处理任务（服务端逻辑）==========
    let counter_in = counter.clone();
    tokio::spawn(async move {
        while let Some((peer, mut stream)) = incoming.next().await {
            // 读取对端发来的计数
            let msg: CountMessage = match stream.read_msg().await {
                Ok(m) => m,
                Err(_) => continue,
            };
            // 取本地和对端的最大值
            let local = counter_in.load(Ordering::SeqCst);
            let new_val = std::cmp::max(local, msg.count);
            counter_in.store(new_val, Ordering::SeqCst);
            println!(
                "📥 Sync from {}: {} -> {}",
                &peer.to_string()[..8],
                msg.count,
                new_val
            );
            // 返回更新后的值给对端
            let _ = stream.write_msg(&CountMessage { count: new_val }).await;
        }
    });

    // ========== 5. 命令通道 ==========
    let (tx, mut rx) = mpsc::channel::<Command>(32);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // ========== 6. Swarm 事件循环任务 ==========
    let peers_clone = peers.clone();
    let counter_loop = counter.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // 处理来自 UI 的命令
                Some(cmd) = rx.recv() => {
                    match cmd {
                        Command::Dial(addr) => {
                            if let Err(e) = swarm.dial(addr.clone()) {
                                eprintln!("❌ Dial failed: {e}");
                            }
                        }
                        Command::Sync(peer_id) => {
                            // 主动发起同步
                            let local = counter_loop.load(Ordering::SeqCst);
                            match control.open_stream(peer_id, COUNTER_PROTOCOL).await {
                                Ok(mut stream) => {
                                    // 发送本地计数
                                    let _ = stream.write_msg(&CountMessage { count: local }).await;
                                    // 读取对端响应
                                    if let Ok(resp) = stream.read_msg::<CountMessage>().await {
                                        // 取最大值更新本地
                                        let new_val = std::cmp::max(local, resp.count);
                                        counter_loop.store(new_val, Ordering::SeqCst);
                                        println!("✅ Synced: {} -> {}", local, new_val);
                                    }
                                }
                                Err(e) => println!("❌ Sync failed: {e}"),
                            }
                        }
                        Command::Increment => {
                            // 本地计数器 +1
                            let new_val = counter_loop.fetch_add(1, Ordering::SeqCst) + 1;
                            println!("➕ Counter: {}", new_val);
                        }
                    }
                }
                // 处理 Swarm 事件
                Some(event) = swarm.next() => {
                    match &event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("👂 Listening on {address}");
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            peers_clone.write().unwrap().insert(*peer_id);
                            println!("✅ Connected to {}", &peer_id.to_string()[..8]);
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            peers_clone.write().unwrap().remove(peer_id);
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    // ========== 7. 初始连接提示 ==========
    println!("Enter address to dial (or press Enter to skip):");
    let addr: String = Input::with_theme(&theme)
        .with_prompt("Multiaddr")
        .allow_empty(true)
        .interact_text()?;
    if !addr.is_empty() {
        if let Ok(addr) = addr.parse::<Multiaddr>() {
            tx.send(Command::Dial(addr)).await?;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }

    // ========== 8. 交互式菜单循环 ==========
    loop {
        let peer_list: Vec<_> = peers.read().unwrap().iter().copied().collect();
        let current = counter.load(Ordering::SeqCst);

        // 构建菜单选项
        let mut options = vec![
            "Dial a peer".to_string(),
            format!("Increment (current: {})", current),
        ];
        for p in &peer_list {
            options.push(format!("Sync with {}", &p.to_string()[..8]));
        }
        options.push("Skip".to_string());
        options.push("Exit".to_string());

        println!(
            "\n--- Counter: {} | Peers: {} ---",
            current,
            peer_list.len()
        );
        let selection = Select::with_theme(&theme)
            .with_prompt("Action")
            .items(&options)
            .default(0)
            .interact()?;

        if selection == 0 {
            // 连接新节点
            let addr: String = Input::with_theme(&theme)
                .with_prompt("Multiaddr")
                .interact_text()?;
            if let Ok(addr) = addr.parse::<Multiaddr>() {
                tx.send(Command::Dial(addr)).await?;
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        } else if selection == 1 {
            // 增加计数
            tx.send(Command::Increment).await?;
        } else if selection == options.len() - 2 {
            // Skip
            continue;
        } else if selection == options.len() - 1 {
            // Exit
            break;
        } else {
            // 与选中的节点同步
            let peer_id = peer_list[selection - 2];
            tx.send(Command::Sync(peer_id)).await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    Ok(())
}

// ==================== 消息编解码 ====================

/// 计数器同步消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountMessage {
    pub count: u64,
}

/// 消息读写扩展 trait
///
/// 使用长度前缀 + JSON 的方式编解码消息：
/// ```text
/// ┌─────────────┬──────────────────────────┐
/// │ 4 bytes     │ N bytes                  │
/// │ (长度 N)    │ (JSON 数据)              │
/// └─────────────┴──────────────────────────┘
/// ```
pub trait MessageExt: AsyncWriteExt + AsyncReadExt + Unpin {
    /// 写入消息：先写 4 字节长度（大端序），再写 JSON 数据
    fn write_msg<M: Serialize>(&mut self, msg: &M) -> impl Future<Output = Result<()>>
    where
        Self: Send,
    {
        let data = serde_json::to_vec(msg);
        async move {
            let data = data?;
            // 写入长度前缀（4 字节，大端序）
            self.write_all(&(data.len() as u32).to_be_bytes()).await?;
            // 写入 JSON 数据
            self.write_all(&data).await?;
            Ok(())
        }
    }

    /// 读取消息：先读 4 字节长度，再读对应长度的 JSON 数据
    fn read_msg<M: for<'de> Deserialize<'de>>(&mut self) -> impl Future<Output = Result<M>>
    where
        Self: Send,
    {
        async move {
            // 读取长度前缀
            let mut len = [0u8; 4];
            self.read_exact(&mut len).await?;
            // 读取 JSON 数据
            let mut buf = vec![0u8; u32::from_be_bytes(len) as usize];
            self.read_exact(&mut buf).await?;
            // 反序列化
            Ok(serde_json::from_slice(&buf)?)
        }
    }
}

/// 为所有实现了 AsyncReadExt + AsyncWriteExt 的类型自动实现 MessageExt
impl<T: AsyncWriteExt + AsyncReadExt + Unpin> MessageExt for T {}
