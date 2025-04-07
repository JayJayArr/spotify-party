use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::user::User;
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Votes(HashMap<String, Vec<User>>);
impl Votes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn vote(&mut self, songid: String, user: User) {
        let votes = match self.0.contains_key(&songid) {
            true => self.0.get_mut(&songid).unwrap(),
            false => &mut self.0.insert(songid, Vec::new()).unwrap(),
        };

        if !votes.contains(&user) {
            votes.push(user);
        }
    }

    pub fn get(&self, id: String) -> Option<&Vec<User>> {
        self.0.get(&id)
    }
    pub fn get_all(&self) -> HashMap<String, Vec<User>> {
        self.0.clone()
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SongSearch {
    searchstring: String,
}
