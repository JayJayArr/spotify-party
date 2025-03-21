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
    // Data(data): Data<Value>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    // check if the user has a
    let songs = db.lock().await.queue.get();
    if songs.len() != 0 {
        let _ = socket.emit("songs", &songs);
    } else {
        match &db.lock().await.client {
            Some(spotclient) => {
                let songs = spotclient.clone().get_user_queue().await.unwrap();
                println!("{:?}", songs);
            }
            None => {
                let _ = socket.emit("songs", "not playing");
                info!("Client was detected as not yet connected");
            }
        }
    }

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

#[derive(Debug)]
pub struct AuthError;
impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthError")
    }
}

pub fn auth_middleware(s: SocketRef, Data(data): Data<Value>) -> Result<(), AuthError> {
    println!("auth middleware called");
    info!(?data, "Socket auth");
    let binding = data.as_map().unwrap().get(0).unwrap().1.clone();
    let token = binding.as_str().unwrap().split(" ");
    info!(?token, "Token");
    // if token != "secret" {
    //     Err(AuthError)
    // } else {
    //     Ok(())
    // }
    Ok(())
}
