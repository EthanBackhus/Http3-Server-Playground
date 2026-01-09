use std::{error::Error, sync::Arc};

use bytes::Bytes;
use h3::server::{Connection, RequestStream, RequestResolver};
use h3_quinn::Connection as QuinnH3Connection;
use http::Response;
use quinn::{Endpoint, ServerConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // create the server config (QUIC requires TLS 1.3)
    let server_config = make_server_config()?;

    // bind a UDP socket
    let addr = "127.0.0.1:3000".parse()?;
    let endpoint = Endpoint::server(server_config, addr)?;

    println!("HTTP/3 server listening on {}", addr);

    // accept QUIC connections
    while let Some(connecting) = endpoint.accept().await {
        tokio::spawn(async move {

            let quinn_conn = match connecting.await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("connection failed: {e}");
                    return;
                }
            };

            println!("new QUIC connection from {}", quinn_conn.remote_address());

            // wrap QUIC connection for HTTP/3
            let h3_conn = QuinnH3Connection::new(quinn_conn);

            // perform HTTP/3 handshake (control streams, QPACK, etc.)
            let mut h3 = match Connection::new(h3_conn).await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("failed to establish h3 connection: {e}");
                    return;
                }
            };

            // accept HTTP/3 request streams
            while let Some(resolver) = match h3.accept().await {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("h3 connection closed: {e}");
                    return;
                }
            } {
                tokio::spawn(async move {
                    if let Err(e) = handle_request(resolver).await {
                        eprintln!("request error: {e}");
                    }
                });
            }
        });
    }

    Ok(())
}

async fn handle_request<C>(
    resolver: RequestResolver<C, Bytes>
) -> Result<(), Box<dyn std::error::Error>> 
where
    C: h3::quic::Connection<Bytes>,
{
    // resolve headers + body stream
    let (req, mut stream) = resolver.resolve_request().await?;

    println!("{} {}", req.method(), req.uri());

    let response = http::Response::builder()
        .status(200)
        .body(())?;

    stream.send_response(response).await?;

    stream
        .send_data(bytes::Bytes::from_static(b"Hello from HTTP/3\n"))
        .await?;

    stream.finish().await?;

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