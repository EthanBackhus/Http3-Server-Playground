# HTTP3 Server Playground

A Rust networking playground exploring modern transport protocols using QUIC (via Quinn) and HTTP/2 (via Hyper)

This repository contains:
- A QUIC server implemented with the quinn library
- A QUIC client that connects, opens bidirectional streams, and exchanges data
- A sample HTTP/2 server using Hyper for comparison and contrast

The goal of this project is to understand how QUIC differs from TCP-based HTTP, how TLS 1.3 is integrated at the transport layer, and how higher-level protocols like HTTP/3 can be layered on top.

## Contents

### QUIC Server
- Listens on UDP
- Performs TLS 1.3 handshakes (required by QUIC)
- Accepts multiple concurrent QUIC connections
- Accepts bidirectional streams per connection
- Echoes data back to the client

### QUIC Client
- Creates a UDP endpoint
- Establishes a QUIC connection to the server
- Opens bidirectional streams
- Sends and receives raw bytes over QUIC

### HTTP/2 server
- Built with Hyper
- Runs over TCP
- Demonstrates the traditional HTTP/2 request/response model
- Serves as a baseline for comparison with QUIC

## Project Structure

```text
src/
├── lib.rs                # Shared helpers (optional)
└── bin/
    ├── servers/
    │   ├── quinn.rs      # QUIC server (UDP + TLS 1.3)
    │   └── http2.rs      # HTTP/2 server using Hyper
    └── clients/
        └── quinn.rs      # QUIC client
```

## Dependencies (high-level)
- **tokio**: async runtime
- **quinn**: QUIC transport implementation
- **rustls**: TLS 1.3 (used by QUIC)
- **rcgen**: self-signed certificate generation (development only)
- **hyper**: HTTP/2 server implementation

## How to Run

### QUIC Server

```bash
cargo run --bin quic-server
```

The server:
- Binds to 127.0.0.1:3000
- Accepts incoming QUIC connections
- Echoes all data received on bidirectional streams

### QUIC Client

```bash
cargo run --bin quic-client
```

The client:
- Connects to the QUIC server
- Opens a bidirectional stream
- Sends a message
- Reads the echoed response

NOTE: The client uses a custom certificate verifier to accept the server's self-signed ceritificate. This is for development only.

### HTTP/2 Server (Hyper)

```bash
cargo run --bin http2-server
```

This server:
- Listens over TCP
- Uses Hyper's HTTP/2 support
- Demonstrates the traditional request/response model over TCP


## Next Steps

Potential next steps built on top of this foundation:
- HTTP/3 frame parsing
- Mapping QUIC streams to HTTP requests
- Integrating Hyper-style service abstractions