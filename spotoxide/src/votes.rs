use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{song::SongId, user::User};
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Votes(HashMap<SongId, Vec<User>>);
impl Votes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, songid: SongId, user: User) {
        let votes = match self.0.contains_key(&songid) {
            true => self.0.get_mut(&songid).unwrap(),
            false => &mut self.0.insert(songid, Vec::new()).unwrap(),
        };
        votes.push(user);
    }
}
