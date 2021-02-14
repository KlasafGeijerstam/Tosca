use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use log::debug;

#[derive(Serialize)]
struct Token<'a> {
    token: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    sub: String,
    exp: u64,
}

pub struct LoginProvider {
    client: Client,
    token_host: String,
    logout_host: String,
}

impl LoginProvider {
    ///! TODO: Cache
    ///! Wraps a Tosca login provider.
    ///! Should store tokens -> user_id mappings as well as token expiry.
    ///! Should implement functions that allow interaction
    ///! with the following endpoints:
    ///! * GET /token
    ///! * POST /logout

    /// Creates a new `LoginProvider`, configured to interact with the provided
    /// Tosca login provider host.
    pub fn new(api_host: &str) -> LoginProvider {
        LoginProvider {
            client: Client::new(),
            token_host: format!("{}/token", api_host),
            logout_host: format!("{}/logout", api_host),
        }
    }

    /// Perform a (cached) lookup, converts a session token to a user-id
    /// TODO: Handle token expiration
    pub async fn lookup(&self, token: &str) -> Result<String> {
        
        debug!("LoginProvider: {} not in cache, performing lookup", token);

        let response = self
            .client
            .get(&self.token_host)
            .json(&Token { token })
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(response.sub)
    }

    /// Logs out the session token both from internal cache, and remote login provider.
    /// TODO
    pub async fn logout(&self, _token: &str) -> Result<()> {
        Ok(())
    }
}
