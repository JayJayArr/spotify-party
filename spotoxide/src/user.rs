use serde::{Deserialize, Serialize};
use socketioxide::socket::Sid;
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct User {
    pub username: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}", self.username)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Usernames(pub HashMap<Sid, User>);

impl Usernames {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}
