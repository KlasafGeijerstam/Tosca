use actix_web::http::header::Header;
use actix_web::{error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use anyhow::bail;
use futures_util::future::{err, ok, Ready};
use reqwest::Client;
use std::marker::PhantomData;

pub struct UserData<L: UserLevel> {
    phantom_level: PhantomData<L>,
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
    pub workspaces: Vec<String>,
}

pub struct UserProvider {
    client: Client,
    api_host: String,
}

/// Maps to data received from LoginProvider
struct RemoteUser {
    pub first_name: String,
    pub last_name: String,
    pub user_level: usize,
    pub workspaces: Vec<String>,
}

impl UserProvider {
    ///! TODO
    ///! Wraps a Tosca user provider.
    ///!

    /// Creates a new `UserProvider` with a given Tosca UserProvider host.
    pub fn new(api_host: &str) -> Self {
        UserProvider {
            client: Client::new(),
            api_host: api_host.into(),
        }
    }

    fn get_user<L: UserLevel>(&self, user_id: &str) -> anyhow::Result<UserData<L>> {
        let RemoteUser {
            first_name,
            last_name,
            user_level,
            workspaces,
        } = self.get_user_from_provider(user_id);

        let required_level = L::new();

        if user_level > required_level.level() {
            bail!(
                "User is missing permissions. Required: {}, actual: {}",
                required_level.level(),
                user_level
            );
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
    fn get_user_from_provider(&self, user_id: &str) -> RemoteUser {
        self.client
            .get(format!("{}/users/{}", self.api_host, user_id));

        //TODO FIX below
        let level = if user_id == "super" {
            0
        } else if user_id == "admin" {
            1
        } else {
            2
        };

        RemoteUser {
            first_name: "Test".into(),
            last_name: "Testsson".into(),
            user_level: level,
            workspaces: vec!["workspace1".into(), "workspace2".into()],
        }
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
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match Authorization::<Bearer>::parse(req) {
            Ok(header) => {
                let header = header.into_scheme();
                let token = header.token();

                // TODO convert token to user_id via LoginProvider app-data
                let provider = req
                    .app_data::<web::Data<UserProvider>>()
                    .expect("No UserProvider configured");

                match provider.get_user(token) {
                    Ok(user) => ok(user),
                    Err(error) => err(ErrorUnauthorized(format!("Unauthorized: {:?}", error))),
                }
            }
            Err(error) => err(ErrorUnauthorized(format!("Unauthorized: {:?}", error))),
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

pub trait UserLevel {
    fn level(&self) -> usize;
    fn new() -> Self
    where
        Self: Sized;
}

pub struct SuperUser;

impl UserLevel for SuperUser {
    fn level(&self) -> usize {
        SUPER_USER
    }

    fn new() -> Self {
        SuperUser {}
    }
}

pub struct AdminUser;

impl UserLevel for AdminUser {
    fn level(&self) -> usize {
        ADMIN_USER
    }

    fn new() -> Self {
        AdminUser {}
    }
}

pub struct NormalUser;

impl UserLevel for NormalUser {
    fn level(&self) -> usize {
        NORMAL_USER
    }

    fn new() -> Self {
        NormalUser {}
    }
}
