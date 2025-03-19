use std::{collections::HashMap, sync::Arc};

use auth::signin_handler;
use axum::routing::{get, post};
use dotenv::dotenv;
use handler::*;
use iohandler::on_connect;
use rnglib::{Language, RNG};
use socketioxide::SocketIoBuilder;
use song_queue::SongQueue;
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::Usernames;
use votes::Votes;

mod auth;
mod handler;
mod iohandler;
mod song;
mod song_queue;
mod user;
mod votes;

pub struct Db {
    users: Usernames,
    votes: Votes,
    queue: SongQueue,
    rng: RNG,
    client_unauth: spotify_rs::client::Client<
        spotify_rs::auth::UnAuthenticated,
        AuthCodeFlow,
        spotify_rs::auth::CsrfVerifier,
    >,
    client: Option<
        spotify_rs::client::Client<
            spotify_rs::auth::Token,
            AuthCodeFlow,
            spotify_rs::auth::NoVerifier,
        >,
    >,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    dotenv().ok();
    let mut store = HashMap::new();
    let client_id =
        std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be specified");
    info!(?client_id, "Spotify Client Id");
    store.insert("client_id", client_id.clone());
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be specified");
    info!(?client_secret, "Spotify Client Secret");
    store.insert("client_secret", client_secret.clone());

    //setup components
    let rng = RNG::from(&Language::Fantasy);
    let queue = SongQueue::new();
    let usernames = Usernames::new();
    let votes = Votes::new();

    //Start the authorization code flow
    let redirect_uri = RedirectUrl::new("http://localhost:3000/redirect".to_owned())?;
    let auto_refresh = true;
    let scopes = vec![
        "user-library-read",
        "playlist-read-private",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-modify-playback-state",
    ];
    let auth_code_flow = AuthCodeFlow::new(client_id, client_secret, scopes);
    let (mut client, url) = AuthCodeClient::new(auth_code_flow, redirect_uri, auto_refresh);
    client.auto_refresh = true;
    let redirecturlstring = url.to_string();

    let db = Db {
        users: usernames,
        votes,
        rng,
        queue,
        client_unauth: client,
        client: None,
    };
    //wrap in an Arc, Mutex
    let dbarc = Arc::new(Mutex::new(db));

    //create io layer
    let (iolayer, io) = SocketIoBuilder::new()
        .with_state(dbarc.clone())
        .build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/signin", post(signin_handler))
        .route("/login", get(|| async { redirecturlstring }))
        .route("/redirect", get(redirect_handler))
        .with_state(dbarc.clone())
        .layer(iolayer)
        // .layer(ConnectMiddleware)
        .layer(CorsLayer::permissive());

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
