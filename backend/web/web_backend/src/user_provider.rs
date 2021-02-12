use actix_web::http::header::Header;
use actix_web::{error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use anyhow::{bail};
use futures_util::future::{err, ok, Ready};

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

pub struct UserData<L: UserLevel> {
    pub level: L,
    pub token: String,
}

pub struct UserProvider {}

impl UserProvider {
    fn get_user<L: UserLevel>(&self, token: &str) -> anyhow::Result<UserData<L>> {
        let actual_level = self.get_user_level(token);
        let required_level = L::new();

        if actual_level.level() > required_level.level() {
            bail!(
                "User is missing permissions. Required: {}, actual: {}",
                required_level.level(),
                actual_level.level()
            );
        }

        Ok(UserData {
            level: required_level,
            token: token.into(),
        })
    }

    /// TODO: Make UserProvider actually use a configured user provider to fetch data
    /// TODO: Cache lookups to user provider
    fn get_user_level(&self, token: &str) -> UserLevels {
        if token == "super" {
            UserLevels::SuperUser
        } else if token == "admin" {
            UserLevels::AdminUser
        } else {
            UserLevels::NormalUser
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

pub enum UserLevels {
    SuperUser,
    AdminUser,
    NormalUser,
}

impl UserLevel for UserLevels {
    fn level(&self) -> usize {
        match self {
            Self::SuperUser => SUPER_USER,
            Self::AdminUser => ADMIN_USER,
            Self::NormalUser => NORMAL_USER,
        }
    }

    fn new() -> Self {
        UserLevels::NormalUser
    }
}

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
