use actix_web::{web, FromRequest, HttpRequest, Error, error::ErrorUnauthorized};
use actix_web::http::header::Header;
use actix_web_httpauth::headers::authorization::{Bearer, Authorization};
use futures_util::future::{ok, err, Ready};
use anyhow::{bail, anyhow};

impl<L> FromRequest for UserData<L> 
where L: UserLevel {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match Authorization::<Bearer>::parse(req) {
            Ok(header) => {
                let header  = header.into_scheme();
                let token = header.token();
                let provider = req.app_data::<web::Data<UserProvider>>()
                    .expect("No UserProvider configured");

                match provider.get_user(token) {
                    Ok(user) => ok(user),
                    Err(error) => err(ErrorUnauthorized(format!("Unauthorized: {:?}", error))),
                }
            },
            Err(error) => {
                err(ErrorUnauthorized(format!("Unauthorized: {:?}", error))) 
            },
        }
    }
}

pub struct UserData<L: UserLevel> {
    pub level: L,
    pub token: String,
}

pub struct UserProvider {

}

impl UserProvider {
    fn get_user<L: UserLevel>(&self, token: &str) -> anyhow::Result<UserData<L>> {
        let actual_level = self.get_user_level();
        let required_level = L::default();

        if actual_level.level() > required_level.level() {
            bail!("User is missing permissions. Required: {}, actual: {}", required_level.level(), actual_level.level());
        }

        Ok(UserData { level: required_level, token: token.into() })
    }

    fn get_user_level(&self) -> impl UserLevel {
        NormalUser::default()
    }
}

#[test]
fn superuser_greater_admin() {
    if SuperUser::default().level() < Admin::default().level() {
        panic!("SuperUser should be greater than Admin")
    }
}

#[test]
fn admin_greater_normal() {
    if Admin::default().level() < NormalUser::default().level() {
        panic!("Admin should be greater than NormalUser")
    }
}

#[test]
fn superuser_greater_normal() {
    if SuperUser::default().level() < NormalUser::default().level() {
        panic!("SuperUser should be greater than NormalUser")
    }
}

pub struct SuperUser;

impl UserLevel for SuperUser {
    fn level(&self) -> usize {
        0
    }
}

impl Default for SuperUser {
    fn default() -> Self {
        SuperUser
    }
}

pub struct Admin;

impl UserLevel for Admin {
    fn level(&self) -> usize {
        1
    }
}

impl Default for Admin {
    fn default() -> Self {
        Admin
    }
}

pub struct NormalUser;

impl UserLevel for NormalUser {
    fn level(&self) -> usize {
        2
    }
}

impl Default for NormalUser {
    fn default() -> Self {
        NormalUser
    }
}

pub trait UserLevel: Default {
    fn level(&self) -> usize;
}

//impl<A, B> PartialEq<UserData<B>> for UserData<A>
//where
    //A: UserLevel,
    //B: UserLevel,
//{
    //fn eq(&self, o: &UserData<B>) -> bool {
        //self.level.level() == o.level.level()
    //}
//}

//impl<A, B> PartialOrd<UserData<B>> for UserData<A>
//where
    //A: UserLevel,
    //B: UserLevel,
//{
    //fn partial_cmp(&self, other: &UserData<B>) -> Option<std::cmp::Ordering> {
        //self.level.level().partial_cmp(&other.level.level())
    //}
//}
