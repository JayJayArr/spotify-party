use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::info;

use crate::Db;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub name: String,
}

pub fn encode_jwt(name: String) -> Result<String, StatusCode> {
    let jwt_token: String = "randomstring".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims { iat, exp, name };
    let secret = jwt_token.clone();

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "randomstring".to_string();

    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn signin_handler(State(db): State<Arc<Mutex<Db>>>) -> Result<Json<String>, StatusCode> {
    let name = db.lock().await.rng.generate_name();
    info!(?name, "Username assigned");
    let token = encode_jwt(name).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(token))
}
