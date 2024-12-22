use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct Song {
    title: String,
    artists: Vec<String>,
    picture: String, //TODO: check if this is really doable as a string
    uri: SongId,     //the Spotify identifier
}
#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct SongId(String);
