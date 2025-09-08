use crate::Db;
use axum::http::StatusCode;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use spotify_rs::endpoint::player::get_user_queue;
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
    let db = &mut db.lock().await;
    if db.client.is_some() {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    let spotify = db.client_unauth.authenticate(code, state).await.unwrap();

    info!("Client connected to spotify");

    // let user_playlists = spotify
    //     .current_user_playlists()
    //     .limit(5)
    //     .get()
    //     .await
    //     .unwrap();
    // info!(?user_playlists, "playlists");
    // let currently_playing = spotify.get_user_queue().await.unwrap();
    let currently_playing = get_user_queue(&spotify).await.unwrap();
    // info!(?currently_playing, "currently_playing");
    db.client = Some(spotify);
    db.queue = currently_playing.into();

    StatusCode::OK
    // let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
}
