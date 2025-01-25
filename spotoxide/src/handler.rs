use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

pub async fn redirect_handler(
    State(clientarc): State<
        Arc<
            spotify_rs::client::Client<
                spotify_rs::auth::UnAuthenticated,
                spotify_rs::AuthCodeFlow,
                spotify_rs::auth::CsrfVerifier,
            >,
        >,
    >,
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
    println!("{:?}", params);
    println!("{:?}", state);
    println!("{:?}", code);
    // let mut spotify = clientarc
    //     .clone()
    //     .authenticate(code, "csrf_token")
    //     .await
    //     .unwrap();

    StatusCode::OK
    // let mut spotify = client.authenticate("auth_code", "csrf_token").await?;
}
