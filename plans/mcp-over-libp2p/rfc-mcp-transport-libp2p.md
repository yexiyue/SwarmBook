# RFC: MCP Transport Binding for libp2p

| Field | Value |
|-------|-------|
| RFC Number | TBD |
| Title | MCP Transport Binding for libp2p |
| Author | [Your Name] |
| Status | Draft |
| Created | 2025-12-26 |
| Updated | 2025-12-26 |

## Summary

This RFC proposes a new transport binding for the Model Context Protocol (MCP) over libp2p streams, enabling peer-to-peer communication between MCP servers and clients.

## Motivation

Current MCP transports (stdio, HTTP, SSE) require either local process communication or centralized server infrastructure. A libp2p-based transport enables:

- **Decentralized deployment**: No central server required
- **NAT traversal**: Connect to MCP servers behind firewalls
- **Peer discovery**: Find MCP services via DHT
- **End-to-end encryption**: Built-in via Noise protocol

### Use Cases

1. **Federated Knowledge Networks**: Nodes share knowledge bases directly
2. **Distributed AI Agents**: Agents collaborate across organizational boundaries
3. **Privacy-Preserving AI**: Direct P2P communication without intermediaries
4. **Resilient Infrastructure**: No single point of failure

## Specification

### Protocol Identifier

```
/mcp/<version>
```

Example: `/mcp/1.0.0`

The version MUST match the MCP specification version being used.

### Message Framing

Messages are framed using length-prefix encoding:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Length (32-bit)                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
|                    JSON-RPC Message (UTF-8)                   |
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

- **Length**: 32-bit unsigned integer in big-endian byte order
- **Message**: UTF-8 encoded JSON-RPC 2.0 message

#### Maximum Message Size

Implementations MUST support messages up to 16 MiB (16,777,216 bytes).

Implementations MAY reject messages exceeding this limit with error code `-32600` (Invalid Request).

### Connection Lifecycle

#### Establishment

1. Client opens a libp2p stream to server with protocol `/mcp/<version>`
2. Server accepts the stream
3. Connection is established

#### Initialization

After stream establishment, standard MCP initialization proceeds:

```
Client                              Server
   |                                   |
   |-------- initialize request ------>|
   |<------- initialize response ------|
   |--- initialized notification ----->|
   |                                   |
```

#### Message Exchange

- Messages flow bidirectionally after initialization
- Multiple requests MAY be in flight simultaneously
- Requests and responses are matched by JSON-RPC `id` field

#### Termination

Either party MAY close the stream at any time.

Implementations SHOULD:
- Complete in-flight requests before closing when graceful shutdown is possible
- Handle abrupt disconnection gracefully

### Error Handling

libp2p errors MUST be mapped to MCP error responses:

| libp2p Error | MCP Error Code | Message |
|--------------|----------------|---------|
| Connection refused | -32000 | "Connection refused" |
| Connection reset | -32000 | "Connection reset" |
| Timeout | -32000 | "Request timeout" |
| Protocol negotiation failed | -32600 | "Protocol not supported" |

## Service Discovery

### DHT Provider Records

MCP servers MAY announce themselves to the Kademlia DHT.

#### Key Derivation

```
key = SHA256("mcp-service:" || service_name)
```

Where `||` denotes concatenation.

#### Provider Record Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "required": ["name", "version"],
  "properties": {
    "name": {
      "type": "string",
      "description": "Service name"
    },
    "version": {
      "type": "string",
      "description": "Service version"
    },
    "capabilities": {
      "type": "array",
      "items": { "type": "string" },
      "description": "MCP capabilities (tools, resources, prompts)"
    },
    "tools": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Available tool names"
    },
    "metadata": {
      "type": "object",
      "description": "Additional metadata"
    }
  }
}
```

#### Example

```json
{
  "name": "knowledge-base",
  "version": "1.0.0",
  "capabilities": ["tools", "resources"],
  "tools": ["search", "query", "summarize"],
  "metadata": {
    "description": "A federated knowledge base service",
    "maintainer": "alice@example.com"
  }
}
```

### Well-Known Keys

| Key | Purpose |
|-----|---------|
| `SHA256("mcp-service:*")` | All MCP services |
| `SHA256("mcp-capability:tools")` | Services providing tools |
| `SHA256("mcp-capability:resources")` | Services providing resources |
| `SHA256("mcp-capability:prompts")` | Services providing prompts |

### Discovery Flow

1. Client computes key for desired service/capability
2. Client queries DHT for providers of that key
3. DHT returns list of PeerIds with provider records
4. Client filters based on provider record metadata
5. Client connects to selected peer(s) using `/mcp/<version>`

## Security Considerations

### Transport Security

libp2p provides transport-level security:

- **Encryption**: All streams encrypted via Noise protocol
- **Authentication**: Peers identified by cryptographic PeerId
- **Integrity**: Message tampering is detected

### Peer Verification

Implementations SHOULD:
- Verify peer identity before exchanging sensitive data
- Support allowlist/blocklist of trusted PeerIds
- Log peer identities for audit purposes

### Resource Limits

Implementations MUST protect against resource exhaustion:

| Resource | Recommended Limit |
|----------|-------------------|
| Concurrent streams per peer | 16 |
| Maximum message size | 16 MiB |
| Request timeout | 30 seconds |
| Idle connection timeout | 5 minutes |

### Tool Execution Security

Tool execution security is out of scope for this transport specification. Implementations SHOULD follow MCP security best practices for tool sandboxing.

## Implementation Notes

### Recommended libp2p Configuration

```
Transports: TCP, QUIC
Security: Noise (XX handshake pattern)
Multiplexing: Yamux
Discovery: Kademlia DHT, mDNS (local)
```

### Connection Pooling

Implementations SHOULD pool connections to reduce handshake overhead for repeated interactions with the same peer.

### Backpressure

Implementations SHOULD implement backpressure mechanisms to handle slow consumers and prevent memory exhaustion.

## Compatibility

### Version Negotiation

When a peer supports multiple MCP versions, protocol negotiation follows libp2p multistream-select:

```
/mcp/1.1.0
/mcp/1.0.0
```

Peers negotiate the highest mutually supported version.

### Fallback

Implementations MAY support fallback to HTTP transport when libp2p connection fails.

## Test Vectors

### Message Encoding

Input (JSON-RPC request):
```json
{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}
```

Encoded (hex):
```
00000038 7b226a736f6e727063223a22322e30222c226964223a312c226d6574686f64223a22746f6f6c732f6c697374222c22706172616d73223a7b7d7d
```

### Key Derivation

Input: `"mcp-service:knowledge-base"`

Output (SHA256, hex):
```
a1b2c3d4e5f6... (actual hash)
```

## References

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [libp2p Specification](https://github.com/libp2p/specs)
- [JSON-RPC 2.0](https://www.jsonrpc.org/specification)
- [Kademlia DHT](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)
- [Noise Protocol](http://noiseprotocol.org/)

## Changelog

- 2025-12-26: Initial draft

## Acknowledgments

This RFC builds upon the work of the MCP and libp2p communities.
