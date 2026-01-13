use bytes::Buf;
use h3::client;
use h3_quinn::Connection;
use quinn::{ClientConfig, Endpoint};
use quinn::crypto::rustls::QuicClientConfig;
use rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use rustls::pki_types::{CertificateDer, ServerName};
use rustls::{Error as TlsError, SignatureScheme};
use std::{error::Error, net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // QUIC endpoint
    let mut endpoint =
        Endpoint::client("0.0.0.0:0".parse::<SocketAddr>()?)?;

    let rustls_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerify))
        .with_no_client_auth();

    let quic_crypto = QuicClientConfig::try_from(rustls_config)?;
    let client_config = ClientConfig::new(Arc::new(quic_crypto));

    endpoint.set_default_client_config(client_config);

    // Establish QUIC connection
    let conn = endpoint
        .connect("127.0.0.1:3000".parse()?, "localhost")?
        .await?;

    println!("QUIC connected");

    // Wrap QUIC connection with h3_quinn
    let quic_conn = Connection::new(conn);

    // Perform HTTP/3 handshake:
    // - open control streams
    // - negotiate settings
    // - initialize QPACK
    let mut h3  = client::new(quic_conn).await?;

    println!("HTTP/3 connected");

    // Send HTTP/3 request (REPLACES open_bi)
    let mut request_stream = h3.1.send_request(
            http::Request::builder()
                .method("GET")
                .uri("https://localhost/")
                .body(())?
        )
        .await?;

    // finish request body
    request_stream.finish().await?;

    // receive response headers
    let response = request_stream.recv_response().await?;

    println!("status: {}", response.status());

    // receive HTTP/3 data frames
    while let Some(mut chunk) = request_stream.recv_data().await? {
        let bytes = chunk.copy_to_bytes(chunk.remaining());
        print!("{}", String::from_utf8_lossy(&bytes));
    }

    Ok(())
}

// TLS verifier
#[derive(Debug)]
struct NoVerify;

impl ServerCertVerifier for NoVerify {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, TlsError> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, TlsError> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ED25519,
        ]
    }
}
