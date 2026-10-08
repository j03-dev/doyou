use reqwest::ClientBuilder;

pub fn builder() -> ClientBuilder {
    let builder = reqwest::Client::builder();

    #[cfg(target_os = "android")]
    let builder = builder.tls_backend_preconfigured(android_tls_config());

    builder
}

#[cfg(target_os = "android")]
fn android_tls_config() -> rustls::ClientConfig {
    use std::sync::Arc;

    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    let mut config = rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(rustls::ALL_VERSIONS)
        .expect("rustls should support all protocol versions")
        .with_root_certificates(root_store)
        .with_no_client_auth();

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    config
}
