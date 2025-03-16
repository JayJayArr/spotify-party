use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use tokio::sync::Mutex;

use crate::Db;

pub async fn redirect_handler(
    State(db): State<Arc<Mutex<Db>>>,
    params: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    if let None = db.lock().await.client {
        return StatusCode::UNAUTHORIZED;
    }
    let state = match params.get("state") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let code = match params.get("code") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    println!("{:?}", state);
    println!("{:?}", code);
    let client_unauth = &mut db.lock().await.client_unauth;
    let mut client = &mut db.lock().await.client;
    client_unauth.auto_refresh = true;
    let spotify = client_unauth
        .clone()
        .authenticate(code, state)
        .await
        .unwrap();
    client = &mut Some(spotify);

    // let user_playlists = spotify
    //     .current_user_playlists()
    //     .limit(5)
    //     .get()
    //     .await
    //     .unwrap();
    // info!(?user_playlists, "playlists");
    // let currently_playing = spotify.get_user_queue().await.unwrap();
    // info!(?currently_playing, "currently_playing");

    StatusCode::OK
    // let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
}
