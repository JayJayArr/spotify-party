use serde::Serialize;

#[derive(Default, Serialize, Clone)]
pub struct Song {
    title: String,
    artists: Vec<String>,
    picture: String, //TODO: check if this is really doable as a string
    uri: String,     //the Spotify identifier
}
