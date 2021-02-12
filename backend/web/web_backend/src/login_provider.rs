use reqwest::Client;
use anyhow::Result;

/// To be sent to the LoginProvider
struct Token {
    token: String
}

pub struct LoginProvider {
    client: Client,
    api_host: String,
}

impl LoginProvider {
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
            api_host: api_host.into(),
        }
    }

    /// TODO &str should perhaps be a String, or Cow<&str>
    /// Perform a (cached) lookup, convert the session token to a user-id
    pub async fn lookup<'a>(&self, token: &'a str) -> Result<&'a str> {
        todo!("Implement..")
    }

    /// TODO
    /// Logs out the session token both from internal cache, and remote login provider.
    pub async fn logout(&self, token: &str) -> Result<()> {
        todo!("Implement")
    }
}
