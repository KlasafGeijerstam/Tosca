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
use std::marker::PhantomData;
use std::pin::Pin;

#[derive(Debug)]
pub struct UserData<L: UserLevel> {
    phantom_level: PhantomData<L>,
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
    pub async fn get_user<L: UserLevel>(&self, user_id: &str) -> anyhow::Result<UserData<L>> {
        let RemoteUser {
            first_name,
            last_name,
            user_level,
            workspaces,
        } = self.get_user_from_provider(user_id).await?;

        let required_level = L::new();

        if user_level > required_level.level() {
            bail!(
                "User lacks permissions. Required: {}, actual: {}",
                required_level.level(),
                user_level
            )
        }

        Ok(UserData {
            phantom_level: PhantomData,
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
impl<L> FromRequest for UserData<L>
where
    L: UserLevel,
{
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

#[test]
fn superuser_less_admin() {
    if SuperUser::new().level() > AdminUser::new().level() {
        panic!("SuperUser should be less than AdminUser")
    }
}

#[test]
fn admin_less_normal() {
    if AdminUser::new().level() > NormalUser::new().level() {
        panic!("AdminUser should be less than NormalUser")
    }
}

#[test]
fn superuser_less_normal() {
    if SuperUser::new().level() > NormalUser::new().level() {
        panic!("SuperUser should be less than NormalUser")
    }
}

const SUPER_USER: usize = 0;
const ADMIN_USER: usize = 1;
const NORMAL_USER: usize = 2;

pub trait UserLevel: std::fmt::Debug {
    fn level(&self) -> usize;
    fn new() -> Self
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct SuperUser;

impl UserLevel for SuperUser {
    fn level(&self) -> usize {
        SUPER_USER
    }

    fn new() -> Self {
        SuperUser {}
    }
}

#[derive(Debug)]
pub struct AdminUser;

impl UserLevel for AdminUser {
    fn level(&self) -> usize {
        ADMIN_USER
    }

    fn new() -> Self {
        AdminUser {}
    }
}

#[derive(Debug)]
pub struct NormalUser;

impl UserLevel for NormalUser {
    fn level(&self) -> usize {
        NORMAL_USER
    }

    fn new() -> Self {
        NormalUser {}
    }
}
