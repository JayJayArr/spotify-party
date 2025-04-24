use rnglib::RNG;
use spotify_rs::AuthCodeFlow;

use crate::{song_queue::SongQueue, user::Usernames, votes::Votes};

pub struct Db {
    pub users: Usernames,
    pub votes: Votes,
    pub queue: SongQueue,
    pub rng: RNG,
    pub client_unauth: spotify_rs::client::Client<
        spotify_rs::auth::UnAuthenticated,
        AuthCodeFlow,
        spotify_rs::auth::CsrfVerifier,
    >,
    pub client: Option<
        spotify_rs::client::Client<
            spotify_rs::auth::Token,
            AuthCodeFlow,
            spotify_rs::auth::NoVerifier,
        >,
    >,
}

impl Db {
    pub fn is_healthy(&self) -> bool {
        !self.rng.generate_name().to_string().is_empty()
    }
}
