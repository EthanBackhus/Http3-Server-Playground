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

    let mut endpoint =
        Endpoint::client("0.0.0.0:0".parse::<SocketAddr>()?)?;

    // build the rustls client config
    let rustls_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(NoVerify))
        .with_no_client_auth();

    // convert from rustls to Quinnls client config
    let quic_crypto = QuicClientConfig::try_from(rustls_config)?;

    // wrap into quinn client config
    let client_config = ClientConfig::new(Arc::new(quic_crypto));
    endpoint.set_default_client_config(client_config);

    // connect to the QUIC server
    let conn = endpoint
        .connect("127.0.0.1:3000".parse()?, "localhost")?
        .await?;

    println!("connected");

    // Open a bidirectional stream
    let (mut send, mut recv) = conn.open_bi().await?;

    send.write_all(b"hello over QUIC").await?;

    // protocol-level FIN
    send.finish()?;

    // read echoed response
    let response = recv.read_to_end(usize::MAX).await?;
    println!("response: {}", String::from_utf8_lossy(&response));

    Ok(())
}

// For development only, do not use in production
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
