use std::sync::Arc;

use rmpv::Value;
use rnglib::RNG;
use serde_json::json;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
};
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    Db,
    song::Song,
    song_queue::SongQueue,
    user::{User, Usernames},
    votes::Votes,
};

pub async fn on_connect(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    Data(data): Data<Value>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    info!(?data, "Socket auth");
    let songs = db.lock().await.queue.get();
    let _ = socket.emit("songs", &songs);

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
