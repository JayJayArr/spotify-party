use std::collections::HashMap;

use axum::{response::Redirect, routing::get};
use dotenv::dotenv;
use rmpv::Value;
use rnglib::{Language, RNG};
use serde_json::json;
use socketioxide::{
    extract::{Data, SocketRef, State, TryData},
    SocketIo, SocketIoBuilder,
};
use song::Song;
use song_queue::SongQueue;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::{User, Usernames};
use votes::Votes;
mod song;
mod song_queue;
mod user;
mod votes;

fn on_connect(socket: SocketRef) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
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
        |socket: SocketRef,
         Data::<Value>(_data),
         State(rng): State<RNG>,
         State(mut users): State<Usernames>| {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    dotenv().ok();
    let mut store = HashMap::new();
    let client_id =
        std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be specified");
    info!(?client_id, "Spotify Client Id");
    store.insert("client_id", client_id);
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be specified");
    info!(?client_secret, "Spotify Client Secret");
    store.insert("client_secret", client_secret);
    let username = std::env::var("SPOTIFY_USERNAME").expect("SPOTIFY_USERNAME must be specified");
    info!(?username, "Spotify username");
    store.insert("client_username", username);
    let password = std::env::var("SPOTIFY_PASSWORD").expect("SPOTIFY_PASSWORD must be specified");
    info!(?password, "Spotify username");
    store.insert("client_password", password);
    //Start the authorization code flow
    let scope = "user-read-currently-playing user-read-playback-state user-modify-playback-state";
    let client = reqwest::Client::new();

    let res = client
        .get("https://api.spotify.com/authorize?")
        .send()
        .await;
    // let mut authroute = "https://api.spotify.com/authorize?".to_owned();
    // authroute.push_str("response_type=code");
    match res {
        Ok(content) => info!(?content, "auth content"),
        Err(err) => info!(?err, "auth err"),
    }

    let rng = RNG::from(&Language::Fantasy);
    let queue = SongQueue::new();
    let usernames = Usernames::new();
    let votes = Votes::new();

    let (layer, io) = SocketIoBuilder::new()
        .with_state(rng)
        .with_state(usernames)
        .with_state(queue)
        .with_state(votes)
        .build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/login",
            get(|| async { Redirect::permanent("https://api.spotify.com/authorize?") }),
        )
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
