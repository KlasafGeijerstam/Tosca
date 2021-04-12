use crate::login_provider::LoginProvider;
use actix_web::http::header::Header;
use actix_web::{
    error::ErrorForbidden, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest,
};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use anyhow::bail;
use futures::Future;
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use std::pin::Pin;

// Available user levels
pub const SUPER_USER: usize = 0;
pub const ADMIN_USER: usize = 1;
pub const NORMAL_USER: usize = 2;

pub type NormalUser = UserData<NORMAL_USER>;
pub type AdminUser = UserData<ADMIN_USER>;
pub type SuperUser = UserData<SUPER_USER>;

#[derive(Debug)]
pub struct UserData<const LEVEL: usize> {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
    pub workspaces: Vec<String>,
}

/// Maps to data received from LoginProvider
#[derive(Deserialize)]
struct RemoteUser {
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
    pub workspaces: Vec<String>,
}

/// TODO: Cache
pub struct UserProvider {
    client: Client,
    api_host: String,
}

impl UserProvider {
    ///! TODO
    ///! Wraps a Tosca user provider.
    ///!
    ///! TODO: GET /workspaces/{workspace\_id}
    ///! TODO: GET /workspaces

    /// Creates a new `UserProvider` with a given Tosca UserProvider host.
    pub fn new(api_host: &str) -> Self {
        UserProvider {
            client: Client::new(),
            api_host: api_host.into(),
        }
    }

    /// Gets UserData for a user_id. Returns `Err` if the user does not exist,
    /// or if the user associated with the user_id lacks permissions (greater permission value) (`L`).
    pub async fn get_user<const LEVEL: usize>(
        &self,
        user_id: &str,
    ) -> anyhow::Result<UserData<LEVEL>> {
        let RemoteUser {
            first_name,
            last_name,
            user_level,
            workspaces,
        } = self.get_user_from_provider(user_id).await?;

        let required_level = LEVEL;

        if user_level > required_level {
            bail!(
                "User lacks permissions. Required: {}, actual: {}",
                required_level,
                user_level
            )
        }

        Ok(UserData {
            user_id: user_id.into(),
            first_name,
            last_name,
            user_level,
            workspaces,
        })
    }

    /// TODO: Make UserProvider actually use a configured user provider to fetch data
    /// TODO: Cache lookups to user provider
    async fn get_user_from_provider(&self, user_id: &str) -> anyhow::Result<RemoteUser> {

        // Lookup cache
        //

        let user = self
            .client
            .get(&format!("{}/users/{}", self.api_host, user_id))
            .send()
            .await?
            .json()
            .await?;

        Ok(user)
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

