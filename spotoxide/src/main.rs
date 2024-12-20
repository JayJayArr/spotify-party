use axum::routing::get;
use rmpv::Value;
use rnglib::{Language, RNG};
use socketioxide::{
    extract::{Data, SocketRef, State, TryData},
    SocketIoBuilder,
};
use song::Song;
use song_queue::SongQueue;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::User;
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
        |socket: SocketRef, TryData::<Song>(song)| {
            info!(?song, "Received event");
            let _ = match song {
                Ok(_song) => socket.emit("message", "got message for song request"),
                Err(_err) => socket.emit("error", "Song is missing or faulty"),
            };
        },
    );

    socket.on(
        "request-username",
        |socket: SocketRef,
         Data::<Value>(data),
         State(rng): State<RNG>,
         State(mut users): State<Vec<User>>| {
            let name = rng.generate_name();
            info!(?data, "Request for username ");
            users.push(User {
                username: name.clone(),
                socket: socket.clone(),
            });
            info!(?name, "Username assigned");
            socket.emit("username", &name).ok();
        },
    );
    socket.on_disconnect(|socket: SocketRef, State(mut users): State<Vec<User>>| {
        //remove the disconnected socket from the users Vec
        users.retain(|x| x.socket != socket);
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let rng = RNG::from(&Language::Fantasy);
    let queue = SongQueue::new();
    let users: Vec<User> = Vec::new();
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
