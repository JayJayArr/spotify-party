use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use axum::{
    Json,
    body::Body,
    extract::{Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::info;

use crate::Db;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub name: String,
}
#[derive(Debug)]
pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}
impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError")
    }
}

pub fn encode_jwt(name: String) -> Result<String, StatusCode> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be specified");

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims { iat, exp, name };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be specified");

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

pub async fn authorize(mut req: Request, next: Next) -> Result<Response<Body>, AuthError> {
    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(AuthError {
                message: "Please add the JWT token to the header".to_string(),
                status_code: StatusCode::FORBIDDEN,
            });
        }
    };

    let mut header = auth_header.split_whitespace();

    let (bearer, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    // Fetch the user details from the database
    // let current_user = match retrieve_user(&token_data.claims.email) {
    //     Some(user) => user,
    //     None => {
    //         return Err(AuthError {
    //             message: "You are not an authorized user".to_string(),
    //             status_code: StatusCode::UNAUTHORIZED,
    //         });
    //     }
    // };
    //
    // req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}

// pub fn retrieve_user() -> Option<User> {}
