use anyhow::{anyhow, bail, Result};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use rand::{thread_rng, Rng};
use reqwest::{self, Client};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

use crate::OpenIDProviderConfig;

/// JWKS should be reloaded once every 24 hours to account for
/// rotating keys and new keys.
const JWKS_RELOAD_TIME: Duration = Duration::from_secs(60 * 60 * 24);

/// Nonces older than NONCE_EXPIRY is removed from storage.
const NONCE_EXPIRY: Duration = Duration::from_secs(60 * 60);

#[derive(Deserialize, Debug)]
pub struct OpenIDCallbackInfo {
    pub state: String,
    pub code: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    id_token: String,
    scope: String,
    token_type: String,
    refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
    pub auth_time: Option<u64>,
    pub nonce: String,

    #[serde(flatten)]
    pub other_claims: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct JWKS {
    keys: Vec<JWTKey>,
}

impl JWKS {
    pub fn get_key(&self, kid: &str) -> Option<&JWTKey> {
        for key in &self.keys {
            if key.kid == kid {
                return Some(key);
            }
        }

        None
    }
}

#[derive(Deserialize, Debug)]
pub struct JWTKey {
    kid: String,
    alg: String,
    n: String,
    e: String,
    x5c: Option<Vec<String>>,

    #[serde(flatten)]
    other: HashMap<String, Value>,
}

fn generate_random_string(len: usize) -> String {
    use rand::distributions::Alphanumeric;

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub struct Session {
    expires_at: SystemTime,
    claims: Claims,
}

pub struct OpenIDProvider {
    config: OpenIDProviderConfig,
    nonces: RwLock<HashMap<String, SystemTime>>,
    session_tokens: RwLock<HashMap<String, Session>>,
    pre_auth_tokens: RwLock<HashMap<String, SystemTime>>,
    jwks: RwLock<JWKS>,
    last_jwks_load: SystemTime,
}

impl OpenIDProvider {
    /// Creates a new OpenIDProvider with the provided configuration
    pub async fn new(config: OpenIDProviderConfig) -> Result<Self> {
        let jwks = config.load_jwks().await?;
        Ok(OpenIDProvider {
            config,
            nonces: RwLock::new(HashMap::new()),
            session_tokens: RwLock::new(HashMap::new()),
            pre_auth_tokens: RwLock::new(HashMap::new()),
            jwks: RwLock::new(jwks),
            last_jwks_load: SystemTime::now(),
        })
    }

    fn generate_nonce(&self) -> String {
        generate_random_string(10)
    }

    async fn load_jwks(&self) -> Result<()> {
        let mut jwks = self.jwks.write().await;
        *jwks = self.config.load_jwks().await?;

        Ok(())
    }

    /// Generate an OpenID login link
    /// Generates a link with embedded state and nonce, with]
    /// a specified callback uri.
    ///
    /// The callback uri must be approved in the OIDC providers
    /// API.
    pub async fn get_login_link(&self, callback_uri: &str) -> String {
        let state_token = generate_random_string(30);
        let nonce = self.generate_nonce();

        self.nonces
            .write()
            .await
            .insert(nonce.clone(), SystemTime::now());

        let token_expiry = SystemTime::now() + Duration::from_secs(360);

        self.pre_auth_tokens
            .write()
            .await
            .insert(state_token.clone(), token_expiry);

        let params = [
            ("client_id", self.config.client_id.as_str()),
            ("response_type", "code"),
            ("scope", "openid email"),
            ("redirect_uri", callback_uri),
            ("state", &state_token),
            ("nonce", &nonce),
        ];

        let client = Client::new();
        client
            .get(self.config.authorization_endpoint())
            .query(&params)
            .build()
            .unwrap()
            .url()
            .to_string()
    }

    async fn exchange_code_for_token(
        &self,
        code: &str,
        redirect_uri: &str,
    ) -> Result<TokenResponse> {
        let client = Client::new();
        let params = [
            ("code", code),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("redirect_uri", redirect_uri),
            ("grant_type", "authorization_code"),
        ];

        let resp: TokenResponse = client
            .post(self.config.token_endpoint())
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    async fn parse_jwt(&self, jwt: &str) -> Result<Claims> {
        let header = decode_header(jwt)?;

        if header.alg != jsonwebtoken::Algorithm::RS256 {
            bail!("Only supports RS256 keys!")
        }

        let last_load = SystemTime::now().duration_since(self.last_jwks_load)?;
        if last_load > JWKS_RELOAD_TIME {
            self.load_jwks().await?;
        }

        let kid = &header.kid.ok_or_else(|| anyhow!("Header is missing kid"))?;

        let jwks = &*self.jwks.read().await;
        let key = jwks
            .get_key(kid)
            .ok_or_else(|| anyhow!("Missing key in JWKS"))?;
        let key = DecodingKey::from_rsa_components(&key.n, &key.e);

        Ok(decode::<Claims>(jwt, &key, &Validation::new(header.alg))?.claims)
    }

    /// Validates a session token and returns the claims for valid tokens
    pub async fn validate_session_token(&self, state: &str) -> Result<Claims> {
        match self.session_tokens.read().await.get(state) {
            Some(session) if SystemTime::now() < session.expires_at => Ok(session.claims.clone()),
            _ => Err(anyhow!("Session token is not valid/expired")),
        }
    }

    /// Verifies an OIDC login callback and retuns the session tokend and TTL for the session.
    /// Checks state and nonces. Trades code for ID-token.
    pub async fn verify_callback(
        &self,
        info: &OpenIDCallbackInfo,
        callback_uri: &str,
    ) -> Result<(String, u64)> {
        match self.pre_auth_tokens.write().await.remove(&info.state) {
            Some(expires_at) if SystemTime::now() < expires_at => true,
            _ => bail!("Token has expired or was never valid"),
        };

        let token_response = self
            .exchange_code_for_token(&info.code, callback_uri)
            .await?;
        let id_token = self.parse_jwt(&token_response.id_token).await?;

        if let None = self.nonces.write().await.remove(&id_token.nonce) {
            bail!("Invalid nonce, possibly a replay attack")
        }

        let new_expiry = UNIX_EPOCH + Duration::from_secs(id_token.exp);
        let expires_in = id_token.exp
            - SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();

        self.session_tokens.write().await.insert(
            info.state.clone(),
            Session {
                claims: id_token,
                expires_at: new_expiry,
            },
        );

        Ok((info.state.clone(), expires_in))
    }

    /// Logs out (removes a session token) from storage.
    pub async fn logout(&self, token: &str) {
        self.session_tokens.write().await.remove(token);
    }

    /// Cleans up expired session tokens and nonces.
    /// Should be called periodically to clean up old data.
    pub async fn cleanup(&self) {
        let clean_sessions = async {
            let mut sessions = self.session_tokens.write().await;
            let now = SystemTime::now();
            sessions.retain(|_, v| now < v.expires_at);
        };

        let clean_tokens = async {
            let mut tokens = self.pre_auth_tokens.write().await;
            let now = SystemTime::now();
            tokens.retain(|_, v| now < *v);
        };

        let clean_nonces = async {
            let mut nonces = self.nonces.write().await;
            let now = SystemTime::now();
            nonces.retain(|_, v| now.duration_since(*v).unwrap() < NONCE_EXPIRY);
        };

        tokio::join!(clean_sessions, clean_tokens, clean_nonces);
    }
}
