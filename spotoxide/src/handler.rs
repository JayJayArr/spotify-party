use crate::Db;
use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use spotify_rs::endpoint::player::get_user_queue;
use spotify_rs::{AuthCodePkceClient, RedirectUrl};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

pub async fn redirect_handler(
    State(db): State<Arc<Mutex<Db>>>,
    params: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    info!("Starting client auth");
    let state = match params.get("state") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let code = match params.get("code") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let db = db.lock().await;
    if db.client.is_some() {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    let client_id =
        std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be specified");
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be specified");
    let redirect_uri = RedirectUrl::new("http://localhost:3000/redirect".to_owned()).unwrap();
    let auto_refresh = true;
    let scopes = vec![
        "user-library-read",
        "playlist-read-private",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-modify-playback-state",
    ];
    let (mut client, url) = AuthCodePkceClient::new(client_id, scopes, redirect_uri, auto_refresh);
    client.auto_refresh = true;
    let spotify = client.authenticate(code, state).await.unwrap();

    info!("Client connected to spotify");

    // let user_playlists = spotify
    //     .current_user_playlists()
    //     .limit(5)
    //     .get()
    //     .await
    //     .unwrap();
    // info!(?user_playlists, "playlists");
    // let currently_playing = spotify.get_user_queue().await.unwrap();
    // let currently_playing = get_user_queue(&spotify).await.unwrap();
    // info!(?currently_playing, "currently_playing");
    // db.client = db.client_unauth;
    // db.client = Some(spotify);
    // db.queue = currently_playing.into();

    StatusCode::OK
}
