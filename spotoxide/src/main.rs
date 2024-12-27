use axum::routing::get;
use rmpv::Value;
use rnglib::{Language, RNG};
use serde_json::json;
use socketioxide::{
    extract::{Data, SocketRef, State, TryData},
    socket::Sid,
    SocketIoBuilder,
};
use song::Song;
use song_queue::SongQueue;
use std::collections::HashMap;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::{User, Usernames};
use votes::Votes;
mod song;
mod song_queue;
mod user;
mod votes;

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();

    socket.on("message", |socket: SocketRef, Data::<Value>(data)| {
        info!(?data, "Received event:");
        socket.emit("message-back", &data).ok();
    });

    socket.on(
        "request-song",
        |socket: SocketRef,
         State(mut votes): State<Votes>,
         State(users): State<HashMap<Sid, User>>,
         TryData::<Song>(song)| {
            let _ = match song {
                Ok(ref _song) => socket.emit("message", "got message for song request"),
                Err(ref _err) => {
                    let _ = socket.emit("error", "Song is missing or faulty");
                    return;
                }
            };
            let username = users.get(&socket.id).unwrap();
            //at this point we can be sure that a song was actually sent
            votes.push(song.unwrap().uri, username.clone());
            socket.emit("votes", &json!(&votes));
        },
    );

    socket.on(
        "request-username",
        |socket: SocketRef,
         Data::<Value>(data),
         State(rng): State<RNG>,
         State(mut users): State<HashMap<Sid, User>>| {
            let name = rng.generate_name();
            info!(?data, "Request for username ");
            users.insert(
                socket.id,
                User {
                    username: name.clone(),
                },
            );
            info!(?name, "Username assigned");
            socket.emit("username", &name).ok();
        },
    );
    socket.on_disconnect(
        |socket: SocketRef, State(mut users): State<HashMap<Sid, User>>| {
            //remove the disconnected socket from the users Vec
            users.remove(&socket.id);
        },
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let rng = RNG::from(&Language::Fantasy);
    let queue = SongQueue::new();
    let users = Usernames::new();
    let votes = Votes::new();

    let (layer, io) = SocketIoBuilder::new()
        .with_state(rng)
        .with_state(users)
        .with_state(queue)
        .with_state(votes)
        .build_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
