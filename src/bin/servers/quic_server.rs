use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::{error::Error, sync::Arc};

// what this server does:
// 1. binds a UDP socket
// 2. performs TLS 1.3 handshake (mandatory for QUIC)
// 3. accepts QUIC connections
// 4. accepts bidirectional QUIC streams
// 5. treats each stream as an independent async task
// 6. echoes bytes back to the client

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let server_config = make_server_config()?;

    // bind a UDP socket
    let addr = "127.0.0.1:3000".parse()?;
    let endpoint = Endpoint::server(server_config, addr)?;

    println!("QUIC server listening on {}", addr);

    // accept QUIC connection
    while let Some(connecting) = endpoint.accept().await {

        // spawn a task to handle QUIC connection
        tokio::spawn(async move {
            let conn = match connecting.await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("connection failed: {e}");
                    return;
                }
            };

            println!("new QUIC connection from {}", conn.remote_address());

            // accept bidirectional stremas on this connection
            loop {
                let (mut send, mut recv) = match conn.accept_bi().await {
                    Ok(stream) => stream,
                    Err(e) => {
                        eprint!("connection closed: {e}");
                        return;
                    }
                };

                // spawn a new task to stream bytes
                tokio::spawn(async move {
                    if let Err(e) = echo_stream(&mut send, &mut recv).await {
                        eprintln!("stream error: {e}");
                    }
                });
            }

        });
    }

    Ok(())
    
}

fn make_server_config() -> Result<ServerConfig, Box<dyn Error>> {
    
    let localhost_alt_names = ["localhost".to_string(), "127.0.0.1".to_string()];

    // generate a self signed ceritificate from localhost
    let certified = rcgen::generate_simple_self_signed(localhost_alt_names)?;

    let cert_der: CertificateDer<'static> = certified.cert.der().clone();

    let key_der: PrivateKeyDer<'static> =
        PrivateKeyDer::from(
            PrivatePkcs8KeyDer::from(
                certified.signing_key.serialize_der() 
            )
        );

    let mut server_config = ServerConfig::with_single_cert(vec![cert_der], key_der)?;

    server_config.transport = Arc::new(quinn::TransportConfig::default());

    Ok(server_config)
}

async fn echo_stream(
    send: &mut quinn::SendStream,
    recv: &mut quinn::RecvStream,
) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 4096];

    loop {
        match recv.read(&mut buf).await? {
            Some(n) => {
                send.write_all(&buf[..n]).await?;
            }
            None => {
                // FIN received: peer is done sending
                break;
            }
        }


    }

    send.finish()?;
    Ok(())
}