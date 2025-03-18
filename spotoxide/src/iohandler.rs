use std::sync::Arc;

use rmpv::Value;
use serde_json::json;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
};
use tokio::sync::Mutex;
use tracing::info;

use crate::{Db, song::Song, user::User};

pub async fn on_connect(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    Data(data): Data<Value>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    let info = data.as_map();
    info!(?data, "Socket auth");
    // check if the user has a
    let songs = db.lock().await.queue.get();
    let _ = socket.emit("songs", &songs);

    socket.on(
        "songs",
        async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
            let queue = &db.lock().await.queue;
            let songs = &queue.get();
            info!("get songs {:?}", songs);
            let _ = socket.emit("songs", songs);
        },
    );

    socket.on(
        "request-song",
        async |socket: SocketRef,
               State(db): State<Arc<Mutex<Db>>>,
               io: SocketIo,
               TryData::<Song>(song)| {
            let _ = match song {
                Ok(ref _song) => socket.emit("message", "got message for song request"),
                Err(ref _err) => {
                    let _ = socket.emit("error", "Song is missing or faulty");
                    //return if the sent Song struct is faulty
                    return;
                }
            };
            let users = &mut db.lock().await.users;
            let username = users.0.get(&socket.id).unwrap();
            //at this point we can be sure that a song was actually sent
            let votes = &mut db.lock().await.votes;
            votes.push(song.unwrap().uri, username.clone());
            //broadcast the updated votes to all clients
            let _ = io.emit("votes", &json!(votes));
        },
    );

    socket.on(
        "request-username",
        async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
            let name = db.lock().await.rng.generate_name();
            let users = &mut db.lock().await.users;
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
    socket.on_disconnect(
        async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
            //remove the disconnected socket from the users Vec

            db.lock().await.users.0.remove(&socket.id);
        },
    );
}
