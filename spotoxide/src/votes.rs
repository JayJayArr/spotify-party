use crate::{song::Song, user::User};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SongSearch {
    pub searchstring: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoteRequest {
    pub uri: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Votes(HashMap<String, (Song, Vec<User>)>);
impl Votes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn vote(&mut self, song: Song, user: User) -> &HashMap<String, (Song, Vec<User>)> {
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

        &self.0
    }

    pub fn get_most_popular(&mut self) -> Option<String> {
        let mut most_popular_song_id: Option<String> = None;
        let mut most_votes: usize = 0;

        for (id, data) in self.0.iter() {
            if data.1.len() > most_votes {
                most_votes = data.1.len();
                most_popular_song_id = Some(id.clone());
            }
        }

        if let Some(songid) = most_popular_song_id {
            most_popular_song_id = Some(songid.clone());
            let _ = self.0.remove(&songid);
        }

        most_popular_song_id
    }

    pub fn get_all(&self) -> Vec<(Song, Vec<User>)> {
        self.0.values().cloned().collect()
    }
}
