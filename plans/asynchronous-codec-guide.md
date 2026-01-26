# asynchronous-codec 使用指南

> 参考文档：[docs.rs/asynchronous-codec](https://docs.rs/asynchronous-codec)、[lib.rs/crates/asynchronous-codec](https://lib.rs/crates/asynchronous-codec)

## 概述

`asynchronous-codec` 是一个用于异步编解码帧的工具库，由 libp2p 维护者 Max Inden 开发。它将 `AsyncRead`/`AsyncWrite` 字节流转换为 `Stream`/`Sink` 帧流。

```
字节流 (AsyncRead/AsyncWrite)
         ↓ Framed + Codec
帧流 (Stream/Sink)
```

## 核心概念

### 1. Encoder / Decoder Trait

```rust
/// 编码器：将消息编码为字节
pub trait Encoder {
    type Item<'a>;              // 要编码的消息类型
    type Error: From<io::Error>;

    fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error>;
}

/// 解码器：从字节解码出消息
pub trait Decoder {
    type Item;                  // 解码出的消息类型
    type Error: From<io::Error>;

    /// 返回 Ok(None) 表示数据不足，需要继续读取
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error>;
}
```

### 2. Framed 包装器

`Framed` 将 I/O 流和 Codec 组合，提供 `Stream` + `Sink` 接口：

```rust
use asynchronous_codec::{Framed, LinesCodec};
use futures::{SinkExt, TryStreamExt};

// 包装 I/O 流
let mut framed = Framed::new(io_stream, LinesCodec);

// 作为 Stream 读取
while let Some(line) = framed.try_next().await? {
    println!("收到: {}", line);
}

// 作为 Sink 写入
framed.send("Hello".to_string()).await?;
```

## 内置 Codec

| Codec | 描述 | 编码格式 |
|-------|------|----------|
| `LengthCodec` | 长度前缀 | 8 字节长度 (u64 big-endian) + 数据 |
| `BytesCodec` | 原样传输 | 无处理 |
| `LinesCodec` | 按行分割 | `\n` 分隔 |
| `JsonCodec` | JSON 编解码 | 需要 `json` feature |
| `CborCodec` | CBOR 编解码 | 需要 `cbor` feature |

## 使用示例

### 示例 1：使用 LengthCodec

```rust
use asynchronous_codec::{Framed, LengthCodec};
use bytes::Bytes;
use futures::{SinkExt, TryStreamExt};

async fn example(stream: impl AsyncRead + AsyncWrite + Unpin) -> Result<()> {
    let mut framed = Framed::new(stream, LengthCodec);

    // 发送数据（自动添加长度前缀）
    framed.send(Bytes::from("Hello")).await?;

    // 接收数据（自动解析长度前缀）
    if let Some(data) = framed.try_next().await? {
        println!("收到 {} 字节", data.len());
    }

    Ok(())
}
```

### 示例 2：自定义 Codec

包装 `LengthCodec` 实现 JSON 消息编解码：

```rust
use asynchronous_codec::{Decoder, Encoder, LengthCodec};
use bytes::{Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;

/// 泛型 JSON Codec，支持不同的编码/解码类型
pub struct JsonLengthCodec<Enc, Dec> {
    inner: LengthCodec,
    _phantom: PhantomData<(Enc, Dec)>,
}

impl<Enc, Dec> Default for JsonLengthCodec<Enc, Dec> {
    fn default() -> Self {
        Self {
            inner: LengthCodec,
            _phantom: PhantomData,
        }
    }
}

impl<Enc: Serialize> Encoder for JsonLengthCodec<Enc, ()> {
    type Item<'a> = &'a Enc;
    type Error = Error;

    fn encode(&mut self, item: Self::Item<'_>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let json = serde_json::to_vec(item)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        self.inner.encode(Bytes::from(json), dst)
    }
}

impl<Dec: for<'de> Deserialize<'de>> Decoder for JsonLengthCodec<(), Dec> {
    type Item = Dec;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.inner.decode(src)? {
            Some(bytes) => {
                let item = serde_json::from_slice(&bytes)
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                Ok(Some(item))
            }
            None => Ok(None),
        }
    }
}
```

### 示例 3：在 libp2p_stream 中使用

```rust
use asynchronous_codec::{Framed, LengthCodec};
use bytes::Bytes;
use futures::{SinkExt, TryStreamExt};
use libp2p_stream::Control;

async fn send_with_codec(control: &mut Control, peer_id: PeerId) -> Result<()> {
    // 打开流
    let stream = control.open_stream(peer_id, MY_PROTOCOL).await?;

    // 用 Framed 包装，自动处理长度前缀
    let mut framed = Framed::new(stream, LengthCodec);

    // 发送消息
    framed.send(Bytes::from("Hello P2P!")).await?;

    // 接收响应
    if let Some(response) = framed.try_next().await? {
        println!("响应: {:?}", response);
    }

    Ok(())
}
```

## LengthCodec 消息格式

```
┌────────────────────┬──────────────────────────┐
│ 8 bytes            │ N bytes                  │
│ 长度 N (u64 BE)    │ 数据                     │
└────────────────────┴──────────────────────────┘
```

**注意**：`LengthCodec` 使用 8 字节 (u64) 长度前缀，而我们在 counter 示例中使用的是 4 字节 (u32)。

## 与手写 MessageExt 的对比

| 特性 | asynchronous-codec | 手写 MessageExt |
|------|-------------------|-----------------|
| 代码量 | 少（使用现成 Codec） | 多（需要自己实现） |
| 灵活性 | 高（可组合 Codec） | 中等 |
| 长度前缀 | 8 字节 (u64) | 可自定义 |
| 缓冲管理 | 自动（BytesMut） | 手动 |
| 背压支持 | 内置 | 需要自己处理 |
| 学习曲线 | 需要理解 Encoder/Decoder | 直观 |

## 何时使用

**推荐使用 asynchronous-codec**：
- 生产环境代码
- 需要复杂的编解码逻辑
- 需要组合多个 Codec
- 需要更好的缓冲管理

**手写 MessageExt 更适合**：
- 教学目的（理解原理）
- 简单场景
- 需要特定的消息格式

## 依赖配置

```toml
[dependencies]
asynchronous-codec = "0.7"
bytes = "1"
futures = "0.3"

# 可选 features
# asynchronous-codec = { version = "0.7", features = ["json", "cbor"] }
```

## 参考链接

- [asynchronous-codec on docs.rs](https://docs.rs/asynchronous-codec)
- [asynchronous-codec on lib.rs](https://lib.rs/crates/asynchronous-codec)
- [源码 - Framed](https://doc.cuprate.org/src/asynchronous_codec/framed.rs.html)
- [源码 - LengthCodec](https://doc.cuprate.org/src/asynchronous_codec/codec/length.rs.html)
