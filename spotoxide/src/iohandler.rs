use std::sync::Arc;

use reqwest::StatusCode;
use rmpv::Value;
use serde_json::json;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
};
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    Db,
    auth::{AuthError, decode_jwt},
    song::Song,
    user::User,
};

pub async fn on_connect(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    // Data(data): Data<Value>,
) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    let songs = db.lock().await.queue.get();
    if !songs.is_empty() {
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

    socket.on_disconnect(
        async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
            //remove the disconnected socket from the users Vec
            let users = &mut db.lock().await.users.0;
            users.remove(&socket.id);
            info!(map = ?users);
        },
    );
}

pub async fn auth_middleware(
    socket: SocketRef,
    Data(data): Data<Value>,
    State(db): State<Arc<Mutex<Db>>>,
) -> Result<(), AuthError> {
    // info!(?data, "Socket auth");
    let binding = match data.as_map() {
        Some(map) => match map.get(0) {
            Some(data) => {
                info!(?map, "Good socket map");
                data.1.clone()
            }
            None => {
                info!(?map, "Socket rejected1");
                return Err(AuthError {
                    message: "Empty Token not allowed".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                });
            }
        },
        None => {
            info!(?data, "Socket rejected2");
            return Err(AuthError {
                message: "No Token provided".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };
    let mut header = binding.as_str().unwrap().split_whitespace();

    let (_bearer, token) = (header.next(), header.next());
    // info!(?token, "Token");
    match decode_jwt(token.unwrap().to_string()) {
        Ok(tokendata) => {
            let users = &mut db.lock().await.users.0;
            users.insert(
                socket.id,
                User {
                    username: tokendata.claims.name,
                },
            );
            info!(map = ?users);
        }
        Err(_err) => {
            info!(?data, "Socket rejected3");
            return Err(AuthError {
                message: "Error decoding token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    }
    // if token != "secret" {
    //     Err(AuthError)
    // } else {
    //     Ok(())
    // }
    Ok(())
}
