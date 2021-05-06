use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub users: HashMap<String, User>,
    pub workspaces: HashMap<String, Workspace>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub user_level: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserWithID {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub user_level: u8,
    pub workspaces: Vec<String>,
}

impl UserWithID {
    pub fn to_user(self) -> User {
        User {
            first_name: self.first_name,
            last_name: self.last_name,
            user_level: self.user_level,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Workspace {
    pub users: Vec<String>,
}

#[derive(Serialize)]
pub struct UserWorkspace {
    pub workspace_id: String,
}
