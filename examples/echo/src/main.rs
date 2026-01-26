//! Echo 协议示例
//!
//! 演示如何使用 libp2p_stream 实现最简单的自定义协议：
//! 客户端发送什么数据，服务端就原样返回什么数据。
//!
//! 运行方式：
//! ```bash
//! cargo run -p echo
//! ```

use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::Result;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use futures::{AsyncReadExt, AsyncWriteExt, StreamExt};
use libp2p::{
    Multiaddr, PeerId, StreamProtocol, SwarmBuilder, noise, swarm::SwarmEvent, tcp, yamux,
};
use libp2p_stream::Control;
use tokio::sync::mpsc;
use tracing::info;

/// 协议标识符，遵循 /name/version 格式
/// 两个节点必须使用相同的协议名才能通信
const ECHO_PROTOCOL: StreamProtocol = StreamProtocol::new("/echo/1.0.0");

/// 从主线程发送到事件循环的命令
enum Command {
    /// 连接到指定地址
    Dial(Multiaddr),
    /// 向指定节点发送消息
    Send { peer_id: PeerId, message: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let theme = ColorfulTheme::default();

    // ========== 1. 创建 Swarm ==========
    // 使用 libp2p_stream::Behaviour 作为协议行为
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
    // Control 是操作流的"遥控器"
    let mut control = swarm.behaviour().new_control();
    // accept() 注册协议并返回入站流的异步迭代器
    let mut incoming_streams = control.accept(ECHO_PROTOCOL)?;

    // 共享的已连接节点列表（用于 UI 显示）
    let peers: Arc<RwLock<HashSet<PeerId>>> = Arc::new(RwLock::new(HashSet::new()));
    let peers_clone = peers.clone();

    // ========== 3. 入站流处理任务（服务端逻辑）==========
    // 在独立任务中处理所有入站的 Echo 请求
    tokio::spawn(async move {
        // 持续监听入站流
        while let Some((peer_id, mut stream)) = incoming_streams.next().await {
            println!("📥 Received stream from {:?}", peer_id);
            let mut buffer = [0; 1024];
            // 循环读取数据直到对端关闭
            while let Ok(n) = stream.read(&mut buffer).await {
                if n == 0 {
                    break; // 对端关闭连接
                }
                println!("🔄 Echo back: {}", String::from_utf8_lossy(&buffer[..n]));
                // Echo: 原样返回收到的数据
                let _ = stream.write_all(&buffer[..n]).await;
            }
        }
    });

    // ========== 4. 命令通道 ==========
    // 用于从主线程（UI）向事件循环发送命令
    let (tx, mut rx) = mpsc::channel::<Command>(32);

    // 开始监听随机端口
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // ========== 5. Swarm 事件循环任务 ==========
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // 处理来自 UI 的命令
                Some(cmd) = rx.recv() => {
                    match cmd {
                        Command::Dial(addr) => {
                            // 连接到指定地址
                            if let Err(e) = swarm.dial(addr.clone()) {
                                eprintln!("❌ Dial failed: {e}");
                            }
                        }
                        Command::Send { peer_id, message } => {
                            // 发送 Echo 消息
                            send_message(&mut control, peer_id, message).await;
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
                            // 记录新连接的节点
                            peers_clone.write().unwrap().insert(*peer_id);
                            println!("✅ Connected to {}", &peer_id.to_string()[..8]);
                        }
                        SwarmEvent::ConnectionClosed { peer_id, .. } => {
                            // 移除断开的节点
                            peers_clone.write().unwrap().remove(peer_id);
                        }
                        _ => {}
                    }
                    info!("{event:?}");
                }
            }
        }
    });

    // ========== 6. 初始连接提示 ==========
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

    // ========== 7. 交互式菜单循环 ==========
    loop {
        let peer_list: Vec<_> = peers.read().unwrap().iter().copied().collect();

        // 构建菜单选项
        let mut options = vec!["Dial a peer".to_string()];
        for p in &peer_list {
            options.push(format!("Send to {}", &p.to_string()[..8]));
        }
        options.push("Skip".to_string());
        options.push("Exit".to_string());

        println!("\n--- Connected peers: {} ---", peer_list.len());
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
        } else if selection == options.len() - 2 {
            // Skip
            continue;
        } else if selection == options.len() - 1 {
            // Exit
            break;
        } else {
            // 向选中的节点发送消息
            let peer_id = peer_list[selection - 1];
            let message: String = Input::with_theme(&theme)
                .with_prompt("Message")
                .interact_text()?;
            tx.send(Command::Send { peer_id, message }).await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    Ok(())
}

/// 发送 Echo 消息（客户端逻辑）
///
/// 1. 打开到目标节点的流
/// 2. 发送消息
/// 3. 读取 Echo 响应
async fn send_message(control: &mut Control, peer_id: PeerId, message: String) {
    // 使用 Control 打开到指定节点的流
    match control.open_stream(peer_id, ECHO_PROTOCOL).await {
        Ok(mut stream) => {
            // 发送消息
            let _ = stream.write_all(message.as_bytes()).await;
            // 读取 Echo 响应（长度与发送相同）
            let mut buf = vec![0; message.len()];
            if stream.read_exact(&mut buf).await.is_ok() {
                println!("✅ Echo: {}", String::from_utf8_lossy(&buf));
            }
        }
        Err(e) => println!("❌ Failed to open stream: {e}"),
    }
}
