use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;
use tokio::sync::Mutex;
use tracing::info;

use crate::Db;

pub async fn redirect_handler(
    State(db): State<Arc<Mutex<Db>>>,
    params: Query<HashMap<String, String>>,
) -> impl IntoResponse {
    info!("Starting client auth");
    // if db.lock().await.client.is_none() {
    //     return StatusCode::UNAUTHORIZED;
    // }
    let state = match params.get("state") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let code = match params.get("code") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let db = &mut db.lock().await;
    if db.client.is_some() {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    info!("Clients all acquired");
    let mut spotify = db
        .client_unauth
        .clone()
        .authenticate(code, state)
        .await
        .unwrap();

    db.client = Some(spotify.clone());
    info!(?spotify, "Client was connected");

    // let user_playlists = spotify
    //     .current_user_playlists()
    //     .limit(5)
    //     .get()
    //     .await
    //     .unwrap();
    // info!(?user_playlists, "playlists");
    let currently_playing = spotify.get_user_queue().await.unwrap();
    // info!(?currently_playing, "currently_playing");
    db.queue = currently_playing.into();

    StatusCode::OK
    // let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
}
