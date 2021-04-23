use anyhow::Result;
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::cache::Cache;

#[derive(Serialize)]
struct Token<'a> {
    token: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    sub: String,
    #[allow(dead_code)]
    exp: u64,
}

pub struct LoginProvider {
    client: Client,
    cache: Cache<String>,
    token_host: String,
    #[allow(dead_code)]
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
            cache: Cache::<String>::builder().with_max_size(1_000).build(),
            token_host: format!("{}/token", api_host),
            logout_host: format!("{}/logout", api_host),
        }
    }

    /// Perform a (cached) lookup, converts a session token to a user-id
    /// TODO: Handle token expiration
    pub async fn lookup(&self, token: &str) -> Result<Arc<String>> {
        if let Some(response) = self.cache.lookup(token.into()).await {
            return Ok(response.clone());
        }

        debug!("LoginProvider: {} not in cache, performing lookup", token);

        let response = self
            .client
            .get(&self.token_host)
            .json(&Token { token })
            .send()
            .await?
            .json::<TokenResponse>()
            .await?
            .sub;

        Ok(self.cache.store(token, response).await)
    }

    /// Logs out the session token both from internal cache, and remote login provider.
    /// TODO
    pub async fn logout(&self, _token: &str) -> Result<()> {
        Ok(())
    }
}
