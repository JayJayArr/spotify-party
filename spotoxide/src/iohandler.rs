use crate::{
    Db,
    auth::{AuthError, decode_jwt},
    song::Song,
    user::User,
    votes::SongSearch,
};
use axum::http::StatusCode;
use rmpv::Value;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
};
use spotify_rs::{endpoint::search::search, model::search::Item};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub async fn on_connect(socket: SocketRef, State(db): State<Arc<Mutex<Db>>>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    let db = &db.lock().await;

    let songs = db.queue.get();
    let votes = db.votes.get_all();

    if !songs.is_empty() {
        let _ = socket.emit("songs", &songs);
    }
    if !votes.is_empty() {
        let _ = socket.emit("votes", &votes);
    }

    if songs.is_empty() && db.client.is_none() {
        let _ = socket.emit("songs", "not playing");
    }

    socket.on("vote", onvote);

    socket.on("search", onsearch);

    socket.on_disconnect(on_disconnect)
}

async fn onvote(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    io: SocketIo,
    TryData(song): TryData<Song>,
) {
    match song {
        Ok(ref song) => info!("song request: {:?}", song),
        Err(ref _err) => {
            let _ = socket.emit("error", "Song is missing or faulty");
            return;
        }
    };
    let db = &mut db.lock().await;
    let username = db.users.0.get(&socket.id).unwrap().clone();
    let votes = db.votes.vote(song.unwrap(), username);

    let _ = io.emit("votes", &votes).await;
}

async fn onsearch(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    Data(searchdata): Data<SongSearch>,
) {
    let db = db.lock().await;
    match &db.client {
        None => {
            let _ = socket.emit("search", "client not ready");
        }
        Some(client) => {
            let response = search(
                searchdata.searchstring,
                &[Item::Album, Item::Artist, Item::Track],
            )
            .get(client)
            .await;
            match response {
                Ok(responsecontent) => {
                    let result = responsecontent.tracks;
                    let _ = socket.emit("search", &result);
                }
                Err(err) => {
                    let _ = socket.emit("search", &err);
                }
            };
        }
    }
}

async fn on_disconnect(socket: SocketRef, State(db): State<Arc<Mutex<Db>>>) {
    //remove the disconnected socket from the users Vec
    let users = &mut db.lock().await.users.0;
    users.remove(&socket.id);
}

pub async fn auth_middleware(
    socket: SocketRef,
    Data(data): Data<Value>,
    State(db): State<Arc<Mutex<Db>>>,
) -> Result<(), AuthError> {
    let binding = match data.as_map() {
        Some(map) => match map.first() {
            Some(data) => data.1.clone(),
            None => {
                return Err(AuthError {
                    message: "Empty Token not allowed".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                });
            }
        },
        None => {
            return Err(AuthError {
                message: "No Token provided".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };
    let mut header = binding.as_str().unwrap().split_whitespace();

    let (_bearer, token) = (header.next(), header.next());
    match decode_jwt(token.unwrap().to_string()) {
        Ok(tokendata) => {
            let users = &mut db.lock().await.users.0;
            users.insert(
                socket.id,
                User {
                    username: tokendata.claims.name,
                },
            );
        }
        Err(_err) => {
            return Err(AuthError {
                message: "Error decoding token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    }
    Ok(())
}
