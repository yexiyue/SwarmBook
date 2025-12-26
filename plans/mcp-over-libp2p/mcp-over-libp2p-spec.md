# MCP over libp2p Transport Specification

> Draft v0.1.0 | 2025-12-25

## Abstract

This document specifies a transport binding for the Model Context Protocol (MCP) over libp2p streams. It enables MCP servers and clients to communicate directly over peer-to-peer networks without centralized infrastructure.

## Status

This is a draft specification for community discussion. It is not yet an official part of the MCP specification.

## 1. Introduction

### 1.1 Motivation

Current MCP transports (stdio, HTTP, SSE) require either local process communication or centralized server infrastructure. A libp2p-based transport enables:

- **Decentralized MCP services**: No central server required
- **NAT traversal**: Connect to MCP servers behind firewalls
- **Peer discovery**: Find MCP services via DHT
- **End-to-end encryption**: Built-in via libp2p Noise protocol

### 1.2 Goals

- Define a standard way to transport MCP JSON-RPC messages over libp2p streams
- Enable service discovery via Kademlia DHT
- Maintain compatibility with existing MCP protocol semantics

### 1.3 Non-Goals

- Modifying MCP protocol semantics
- Defining new MCP methods or capabilities
- Specifying authentication/authorization beyond transport-level identity

## 2. Protocol Identifier

```
/mcp/1.0.0
```

The protocol identifier follows libp2p conventions:
- `/mcp` - Protocol name (Model Context Protocol)
- `/1.0.0` - Version matching MCP specification version

## 3. Message Framing

MCP uses JSON-RPC 2.0 messages. Over libp2p streams, messages are framed using length-prefixed encoding:

```
+------------------+------------------------+
| Length (4 bytes) | JSON-RPC Message       |
| big-endian u32   | UTF-8 encoded          |
+------------------+------------------------+
```

### 3.1 Frame Format

| Field | Size | Description |
|-------|------|-------------|
| Length | 4 bytes | Message length in bytes (big-endian unsigned 32-bit integer) |
| Payload | Variable | UTF-8 encoded JSON-RPC 2.0 message |

### 3.2 Maximum Message Size

Implementations MUST support messages up to 16 MiB (16,777,216 bytes). Implementations MAY reject messages exceeding this limit.

### 3.3 Example

A `tools/list` request:

```json
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
```

Encoded as (hex):
```
00 00 00 38                                     # Length: 56 bytes
7b 22 6a 73 6f 6e 72 70 63 22 3a 22 32 2e 30    # {"jsonrpc":"2.0"
22 2c 22 69 64 22 3a 31 2c 22 6d 65 74 68 6f    # ,"id":1,"metho
64 22 3a 22 74 6f 6f 6c 73 2f 6c 69 73 74 22    # d":"tools/list"
2c 22 70 61 72 61 6d 73 22 3a 7b 7d 7d          # ,"params":{}}
```

## 4. Connection Lifecycle

### 4.1 Connection Establishment

1. Client opens a libp2p stream to server with protocol `/mcp/1.0.0`
2. Server accepts the stream
3. Both parties can now exchange MCP messages

### 4.2 Initialization

After stream establishment, the standard MCP initialization handshake occurs:

```
Client                              Server
   |                                   |
   |-------- initialize request ------>|
   |<------- initialize response ------|
   |                                   |
   |--- initialized notification ----->|
   |                                   |
```

### 4.3 Message Exchange

After initialization, messages flow bidirectionally:
- Client sends requests, server sends responses
- Either party may send notifications
- Multiple requests may be in flight (matched by `id`)

### 4.4 Connection Termination

Either party may close the stream at any time. Implementations SHOULD:
- Complete in-flight requests before closing when possible
- Handle abrupt disconnection gracefully

## 5. Service Discovery

### 5.1 DHT Provider Records

MCP servers MAY announce themselves to the Kademlia DHT using provider records.

**Key derivation:**
```
key = SHA256("mcp-service:" + service_name)
```

**Provider record content:**
```json
{
  "name": "my-knowledge-base",
  "version": "1.0.0",
  "capabilities": ["tools", "resources"],
  "tools": ["search", "query"],
  "metadata": {
    "description": "A knowledge base service"
  }
}
```

### 5.2 Discovery Flow

1. Client computes key for desired service/capability
2. Client queries DHT for providers of that key
3. DHT returns list of PeerIds
4. Client connects to peer and opens MCP stream

### 5.3 Well-Known Keys

| Key | Purpose |
|-----|---------|
| `SHA256("mcp-service:*")` | All MCP services |
| `SHA256("mcp-capability:tools")` | Services with tools |
| `SHA256("mcp-capability:resources")` | Services with resources |
| `SHA256("mcp-capability:prompts")` | Services with prompts |

## 6. Security Considerations

### 6.1 Transport Security

libp2p provides transport-level security via the Noise protocol. All MCP messages are encrypted in transit.

### 6.2 Peer Identity

Each peer has a cryptographic identity (PeerId derived from public key). Implementations SHOULD:
- Verify peer identity before exchanging sensitive data
- Maintain allowlists/blocklists of trusted peers

### 6.3 Resource Limits

Implementations MUST protect against resource exhaustion:
- Limit concurrent streams per peer
- Limit message size (see Section 3.2)
- Implement rate limiting for requests

### 6.4 Tool Execution

MCP tool execution security is out of scope for this transport specification. Implementations SHOULD follow MCP security best practices for tool sandboxing.

## 7. Implementation Notes

### 7.1 Recommended libp2p Configuration

```
Transport: TCP + QUIC
Security: Noise
Multiplexing: Yamux
Discovery: Kademlia DHT + mDNS (local)
```

### 7.2 Rust Implementation

Reference implementation: `rmcp-transport-p2p` crate

```rust
use rmcp::ServiceExt;
use rmcp_transport_p2p::P2pTransport;

// Server
my_service().serve(P2pTransport::new(config)).await?;

// Client
let client = ().serve(P2pTransport::connect(peer_id, config)).await?;
```

## 8. References

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [libp2p Specification](https://github.com/libp2p/specs)
- [JSON-RPC 2.0](https://www.jsonrpc.org/specification)
- [Kademlia DHT](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)

## Appendix A: Protocol Negotiation

When a peer supports multiple MCP versions, protocol negotiation follows libp2p multistream-select:

```
/mcp/1.0.0
/mcp/0.9.0  (hypothetical older version)
```

Peers negotiate the highest mutually supported version.

## Appendix B: Comparison with Other Transports

| Transport | Discovery | NAT Traversal | Encryption | Decentralized |
|-----------|-----------|---------------|------------|---------------|
| stdio | N/A | N/A | N/A | N/A |
| HTTP | Manual | No | TLS (optional) | No |
| SSE | Manual | No | TLS (optional) | No |
| **libp2p** | DHT | Yes | Noise | Yes |

## Changelog

- v0.1.0 (2025-12-25): Initial draft
