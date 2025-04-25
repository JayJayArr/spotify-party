use serde::{Deserialize, Serialize};
use spotify_rs::model::track::Track;

#[derive(Default, Serialize, Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Song {
    title: String,
    artists: Vec<String>,
    picture: String,
    pub uri: String, //the Spotify identifier
}

impl From<Track> for Song {
    fn from(value: Track) -> Self {
        Self {
            title: value.name,
            artists: value
                .artists
                .iter()
                .map(|artist| artist.name.clone())
                .collect(),
            uri: value.uri, // artists: value.artists.into(),
            picture: value.album.images[0].url.clone(),
        }
    }
}
