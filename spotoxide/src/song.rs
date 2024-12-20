use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Clone, Debug, Deserialize)]
pub struct Song {
    title: String,
    artists: Vec<String>,
    picture: String, //TODO: check if this is really doable as a string
    uri: String,     //the Spotify identifier
}
