use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{song::Song, user::User};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Votes(HashMap<String, (Song, Vec<User>)>);
impl Votes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn vote(&mut self, song: Song, user: User) {
        let (_data, users) = match self.0.contains_key(&song.uri) {
            true => self.0.get_mut(&song.uri).unwrap(),
            false => {
                self.0.insert(song.uri.clone(), (song.clone(), Vec::new()));
                self.0.get_mut(&song.uri).unwrap()
            }
        };

        if !users.contains(&user) {
            users.push(user);
        }
    }

    // pub fn get(&self, song: Song) -> Option<&Vec<User>> {
    //     self.0.get(&id)
    // }
    pub fn get_all(&self) -> Vec<(Song, Vec<User>)> {
        self.0.values().cloned().collect()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SongSearch {
    pub searchstring: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    pub uri: String,
}
