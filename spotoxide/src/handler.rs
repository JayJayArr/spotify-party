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
    let state = match params.get("state") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let code = match params.get("code") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    // println!("{:?}", params);
    println!("{:?}", state);
    println!("{:?}", code);
    let client = &mut db.lock().await.client;
    client.auto_refresh = true;
    db.lock().await.client = client.clone().authenticate(code, state).await.unwrap();

    StatusCode::OK
    // let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
}
