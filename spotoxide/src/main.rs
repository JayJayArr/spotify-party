use std::{collections::HashMap, sync::Arc};

use axum::routing::get;
use dotenv::dotenv;
use handler::*;
use rmpv::Value;
use rnglib::{Language, RNG};
use serde_json::json;
use socketioxide::{
    SocketIo, SocketIoBuilder,
    extract::{Data, SocketRef, State, TryData},
};
use song::Song;
use song_queue::SongQueue;
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::{User, Usernames};
use votes::Votes;
mod handler;
mod song;
mod song_queue;
mod user;
mod votes;

fn on_connect(socket: SocketRef, State(queue): State<SongQueue>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    let songs = &queue.get();
    let _ = socket.emit("songs", songs);

    socket.on(
        "songs",
        |socket: SocketRef, State(queue): State<SongQueue>| {
            let songs = &queue.get();
            info!("get songs {:?}", songs);
            let _ = socket.emit("songs", songs);
        },
    );

    socket.on(
        "request-song",
        |socket: SocketRef,
         State(mut votes): State<Votes>,
         io: SocketIo,
         State(users): State<Usernames>,
         TryData::<Song>(song)| {
            let _ = match song {
                Ok(ref _song) => socket.emit("message", "got message for song request"),
                Err(ref _err) => {
                    let _ = socket.emit("error", "Song is missing or faulty");
                    //return if the sent Song struct is faulty
                    return;
                }
            };
            let username = users.0.get(&socket.id).unwrap();
            //at this point we can be sure that a song was actually sent
            votes.push(song.unwrap().uri, username.clone());
            //broadcast the updated votes to all clients
            let _ = io.emit("votes", &json!(votes));
        },
    );

    socket.on(
        "request-username",
        |socket: SocketRef, State(rng): State<RNG>, State(mut users): State<Usernames>| {
            let name = rng.generate_name();
            users.0.insert(
                socket.id,
                User {
                    username: name.clone(),
                },
            );
            info!(?name, "Username assigned");
            socket.emit("username", &name).ok();
        },
    );
    socket.on_disconnect(|socket: SocketRef, State(mut users): State<Usernames>| {
        //remove the disconnected socket from the users Vec
        users.0.remove(&socket.id);
    });
}

pub struct Db {
    users: Usernames,
    votes: Votes,
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

    let (client, url) = AuthCodeClient::new(auth_code_flow, redirect_uri, auto_refresh);
    let db = Db {
        users: usernames.clone(),
        votes: votes.clone(),
        rng: rng.clone(),
        client_unauth: client,
        client: None,
    };
    let redirecturlstring = url.to_string();

    let (iolayer, io) = SocketIoBuilder::new()
        .with_state(rng)
        .with_state(usernames)
        .with_state(queue)
        .with_state(votes)
        .build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/login", get(|| async { redirecturlstring }))
        .route("/redirect", get(redirect_handler))
        .with_state(Arc::new(Mutex::new(db)))
        .layer(iolayer)
        .layer(CorsLayer::permissive());

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
