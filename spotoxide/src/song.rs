use serde::Serialize;

#[derive(Default, Serialize)]
pub struct Song {
    title: String,
    artists: Vec<String>,
    picture: String, //TODO: check if this is really doable
    uri: String,     //the Spotify identifier
}
