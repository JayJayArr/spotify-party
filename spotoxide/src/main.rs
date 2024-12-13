use axum::routing::get;
use rmpv::Value;
use rnglib::{Language, RNG};
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use song_queue::SongQueue;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
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

    socket.on("message-with-ack", |Data::<Value>(data), ack: AckSender| {
        info!(?data, "Received event");
        ack.send(&data).ok();
    });

    socket.on(
        "request-username",
        |socket: SocketRef, Data::<Value>(data)| {
            info!(?data, "Request for username ");
            socket.emit("username", "test").ok();
        },
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let rng = RNG::try_from(&Language::Fantasy).unwrap();
    //TODO: inject a reference to rng into every socket using extension
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer);

    info!("Starting server");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
