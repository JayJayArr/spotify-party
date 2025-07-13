use crate::{song_queue::SongQueue, user::Usernames, votes::Votes};
use rnglib::RNG;
use spotify_rs::{AuthCodeFlow, Unauthenticated};

pub struct Db {
    pub users: Usernames,
    pub votes: Votes,
    pub queue: SongQueue,
    pub rng: RNG,
    pub client_unauth: spotify_rs::client::Client<Unauthenticated, AuthCodeFlow>,
    pub client: Option<spotify_rs::client::Client<spotify_rs::Token, AuthCodeFlow>>,
}

impl Db {
    pub fn is_healthy(&self) -> bool {
        !self.rng.generate_name().to_string().is_empty()
    }
}
