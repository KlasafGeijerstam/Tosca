use crate::cache::Cache;
use crate::login_provider::LoginProvider;
use actix_web::http::header::Header;
use actix_web::{
    error::ErrorForbidden, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use anyhow::bail;
use anyhow::Result;
use futures::Future;
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::sync::Arc;

// Available user levels
pub const SUPER_USER: usize = 0;
pub const ADMIN_USER: usize = 1;
pub const NORMAL_USER: usize = 2;

pub type NormalUser = UserData<NORMAL_USER>;
pub type AdminUser = UserData<ADMIN_USER>;
pub type SuperUser = UserData<SUPER_USER>;

#[derive(Debug)]
pub struct UserData<const LEVEL: usize> {
    user: Arc<(User, Workspaces)>,
}

impl<const LEVEL: usize> UserData<LEVEL> {
    pub fn user_id(&self) -> &str {
        &self.user.0.user_id
    }

    pub fn first_name(&self) -> &str {
        &self.user.0.first_name
    }

    pub fn last_name(&self) -> &str {
        &self.user.0.last_name
    }

    pub fn level(&self) -> usize {
        self.user.0.user_level
    }

    pub fn user(&self) -> &User {
        &self.user.0
    }

    pub fn workspaces(&self) -> &Vec<String> {
        &self.user.1
    }
}

pub type Workspaces = Vec<String>;

#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
}

/// Maps to data received from LoginProvider
#[derive(Deserialize)]
struct RemoteUser {
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
    pub workspaces: Vec<String>,
}
pub struct UserProvider {
    client: Client,
    api_host: String,
    cache: Cache<(User, Workspaces)>,
}

impl UserProvider {
    ///! Wraps a Tosca user provider.

    /// Creates a new `UserProvider` with a given Tosca UserProvider host.
    pub fn new(api_host: &str) -> Self {
        UserProvider {
            client: Client::new(),
            api_host: api_host.into(),
            cache: Cache::<(User, Workspaces)>::builder()
                .with_max_size(1_000)
                .build(),
        }
    }

    /// Gets a user from a user id
    pub async fn get(&self, user_id: &str) -> anyhow::Result<Arc<(User, Workspaces)>> {
        Ok(self.get_user::<NORMAL_USER>(user_id).await?.user)
    }

    /// Gets UserData for a user_id. Returns `Err` if the user does not exist,
    /// or if the user associated with the user_id lacks permissions (greater permission value) (`L`).
    pub async fn get_user<const LEVEL: usize>(
        &self,
        user_id: &str,
    ) -> anyhow::Result<UserData<LEVEL>> {
        let uw = self.get_user_from_provider(user_id).await?;

        let required_level = LEVEL;

        if uw.0.user_level > required_level {
            bail!(
                "User lacks permissions. Required: {}, actual: {}",
                required_level,
                uw.0.user_level
            )
        }

        Ok(UserData { user: uw })
    }

    async fn get_user_from_provider(
        &self,
        user_id: &str,
    ) -> anyhow::Result<Arc<(User, Workspaces)>> {
        if let Some(response) = self.cache.lookup(user_id).await {
            return Ok(response);
        }

        let user: RemoteUser = self
            .client
            .get(&format!("{}/users/{}", self.api_host, user_id))
            .send()
            .await?
            .json()
            .await?;

        let workspaces = user.workspaces;
        let user = User {
            user_id: user_id.into(),
            first_name: user.first_name,
            last_name: user.last_name,
            user_level: user.user_level,
        };

        Ok(self.cache.store(user_id, (user, workspaces)).await)
    }
}

/// Parses a UserData by Bearer token and UserProvider lookup.
/// Requires that the App has a `UserProvider` registered as data.
/// Requires that the request has a `Authorization` header of type `Bearer` set.
impl<const LEVEL: usize> FromRequest for UserData<LEVEL> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match Authorization::<Bearer>::parse(req) {
            Ok(header) => {
                let header = header.into_scheme();
                let token = header.token().to_owned();

                let login_provider = req
                    .app_data::<web::Data<LoginProvider>>()
                    .expect("No LoginProvider configured")
                    .clone();

                let user_provider = req
                    .app_data::<web::Data<UserProvider>>()
                    .expect("No UserProvider configured")
                    .clone();

                Box::pin(async move {
                    debug!("Looking up token {}", token);

                    let user_id = match login_provider.lookup(&token).await {
                        Ok(user_id) => user_id,
                        Err(error) => {
                            return Err(ErrorUnauthorized(format!("Unauthorized: {:?}", error)))
                        }
                    };

                    debug!("Token matches user {}", user_id);

                    match user_provider.get_user(&user_id).await {
                        Ok(user) => Ok(user),
                        Err(error) => Err(ErrorForbidden(format!("Unauthorized: {:?}", error))),
                    }
                })
            }
            Err(error) => {
                Box::pin(
                    async move { Err(ErrorUnauthorized(format!("Unauthorized: {:?}", error))) },
                )
            }
        }
    }
}

pub mod user_field {
    use super::{NORMAL_USER, User};
    use crate::api::UserProvider;
    use actix_web::{Error, HttpResponse};
    use log::error;
    use serde::Serialize;

    use UserField::*;

    pub async fn lookup(user_id: &str, user_provider: &UserProvider) -> anyhow::Result<User> {
        Ok(user_provider.get(user_id).await?.0.clone())
    }

    #[derive(Serialize, Debug)]
    #[serde(untagged)]
    pub enum UserField {
        Username(String),
        FullUser(User),
    }

    impl From<String> for UserField {
        fn from(user_id: String) -> Self {
            Username(user_id)
        }
    }

    impl UserField {
        pub async fn lookup(&mut self, user_provider: &UserProvider) -> Result<(), Error> {
            if let Username(user_id) = &self {
                *self = FullUser(lookup(&user_id, &user_provider).await.map_err(|e| {
                    error!("Failed to lookup user: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                })?);
            }
            Ok(())
        }
    }
    
    pub fn unknown_user() -> UserField {
        FullUser(User {
            user_id: "unknown_user".into(),
            first_name: "Unkown".into(),
            last_name: "User".into(),
            user_level: NORMAL_USER,
        })
    }
}
