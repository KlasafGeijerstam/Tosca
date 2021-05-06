use anyhow::Result;
use reqwest::{self, Client};
use serde::Deserialize;

use crate::Jwks;

#[derive(Deserialize, Debug)]
struct DiscoveryFields {
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub jwks_uri: String,
}

#[derive(Deserialize, Debug)]
struct ConfigFile {
    public_host: String,
    port: u16,
    certificate_file: String,
    key_file: String,

    discovery_server: String,
    client_id: String,
    client_secret: String,
    client_endpoint: String,
}

pub struct Config {
    pub public_host: String,
    pub port: u16,
    pub provider: OpenIdProviderConfig,
    pub client_endpoint: String,
    pub certificate_file: String,
    pub key_file: String,
}

/// Represents a OpenIdProvider config file
impl Config {
    /// Loads a Config from a TOML file.
    pub async fn from_config_file(path: &str) -> Result<Self> {
        let config: ConfigFile = toml::from_str(
            &std::fs::read_to_string(path).expect("Failed to open OpenId Connect config file"),
        )?;

        let provider = OpenIdProviderConfig::new(
            &config.client_id,
            &config.client_secret,
            &config.discovery_server,
        )
        .await?;

        Ok(Config {
            public_host: config.public_host,
            port: config.port,
            certificate_file: config.certificate_file,
            key_file: config.key_file,
            provider,
            client_endpoint: config.client_endpoint,
        })
    }
}

pub struct OpenIdProviderConfig {
    pub discovery_server: String,
    pub client_id: String,
    pub client_secret: String,
    discovery_data: DiscoveryFields,
}

impl OpenIdProviderConfig {
    /// Loads the Jwks from the Jwks server
    pub async fn load_jwks(&self) -> Result<Jwks> {
        Ok(reqwest::get(&self.discovery_data.jwks_uri)
            .await?
            .json()
            .await?)
    }

    pub fn authorization_endpoint(&self) -> &str {
        &self.discovery_data.authorization_endpoint
    }

    pub fn token_endpoint(&self) -> &str {
        &self.discovery_data.token_endpoint
    }

    pub fn jwks_uri(&self) -> &str {
        &self.discovery_data.jwks_uri
    }

    /// Creates a new OpenIdProviderConfig
    /// The config loads the discovery data from the provided discovery server URI.
    pub async fn new(client_id: &str, client_secret: &str, discovery_server: &str) -> Result<Self> {
        let client = Client::new();
        let fields: DiscoveryFields = client.get(discovery_server).send().await?.json().await?;

        Ok(OpenIdProviderConfig {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            discovery_data: fields,
            discovery_server: discovery_server.into(),
        })
    }
}
