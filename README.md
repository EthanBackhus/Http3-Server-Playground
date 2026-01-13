# HTTP3 Server Playground

A Rust networking playground exploring modern transport protocols using HTTP/3 (via h3) over QUIC (via Quinn), and HTTP/2 (via Hyper)

This repository contains:
- A HTTP/3 server implemented with h3 + quinn
- A HTTP/3 client implemented with h3 + quinn
- A QUIC server implemented with the quinn library
- A QUIC client that connects, opens bidirectional streams, and exchanges data
- A sample HTTP/2 server using Hyper for comparison and contrast

The goal of this project is to understand how QUIC differs from TCP-based HTTP, how TLS 1.3 is integrated at the transport layer, and how higher-level protocols like HTTP/3 can be layered on top.

## Contents

### HTTP/3 Server
- Listens on UDP
- Performs TLS 1.3 handshakes (required by QUIC)
- Accepts multiple concurrent HTTP/3 connections
- Accepts each bidirectional HTTP/3 request stream
- Responds with a 200 OK and "Hello from HTTP/3" as the body

### HTTP/3 Client
- Connects to a UDP endpoint
- Sets up a TLS config
- Establishes QUIC connection to server
- Performs HTTP/3 handshake
- Opens new bidirectional HTTP/3 request stream
- Finishes stream
- Prints response headers and payload to console

### QUIC Server
- Listens on UDP
- Performs TLS 1.3 handshakes (required by QUIC)
- Accepts multiple concurrent QUIC connections
- Accepts bidirectional streams per connection
- Echoes data back to the client

### QUIC Client
- Connects to a UDP endpoint
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
├── lib.rs                # Shared helpers (WIP)
└── bin/
    ├── servers/
    │   ├── quic_server.rs      # QUIC server (UDP + TLS 1.3)
    │   ├── http2_server.rs     # HTTP/2 server using Hyper
    │   └── h3_quinn_server     # HTTP/3 server over h3 + quinn
    └── clients/
        ├── quic_client.rs      # QUIC client
        └── h3_quinn_client.rs  # HTTP/3 Client over h3 + quinn
```

## Dependencies (high-level)
- **tokio**: async runtime
- **quinn**: QUIC transport implementation
- **rustls**: TLS 1.3 (used by QUIC)
- **rcgen**: self-signed certificate generation (development only)
- **hyper**: HTTP/2 server implementation

## How to Run

### HTTP/3 Server
```bash
cargo run --bin h3-quinn-server
```

The server:
- Binds to 127.0.0.1:3000
- Accepts incoming HTTP/3 Connections
- Handles each bidirectional HTTP/3 request stream
- Responds with a 200 OK and "Hello from HTTP/3" as the body
- Closes each stream after sending the response

### HTTP/3 Client
```bash
cargo run --bin h3-quinn-client
```

The client:
- Connects to UDP endpoint
- Sets up TLS config
- Establishs QUIC connection
- Performs HTTP/3 handshake
- Opens a bidirectional HTTP/3 request stream
- Prints response header to console
- Prints payload from HTTP/3 data frame to console

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