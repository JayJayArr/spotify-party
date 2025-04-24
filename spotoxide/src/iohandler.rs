use std::sync::Arc;

use axum::http::StatusCode;
use rmpv::Value;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef, State, TryData},
};
use spotify_rs::model::search::Item;
use tokio::sync::Mutex;
use tracing::info;

use crate::{
    Db,
    auth::{AuthError, decode_jwt},
    song::Song,
    user::User,
    votes::SongSearch,
};

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

    // socket.on(
    //     "songs",
    //     async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
    //         let queue = &db.lock().await.queue;
    //         let songs = &queue.get();
    //         let _ = socket.emit("songs", songs);
    //     },
    // );

    socket.on("vote", onvote);

    socket.on("search", onsearch);

    socket.on_disconnect(
        async |socket: SocketRef, State(db): State<Arc<Mutex<Db>>>| {
            //remove the disconnected socket from the users Vec
            let users = &mut db.lock().await.users.0;
            users.remove(&socket.id);
        },
    );
}

async fn onvote(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    io: SocketIo,
    TryData(song): TryData<Song>,
) {
    match song {
        Ok(ref _song) => info!("got message for song request"),
        Err(ref _err) => {
            let _ = socket.emit("error", "Song is missing or faulty");
            return;
        }
    };
    let db = &mut db.lock().await;
    let username = db.users.0.get(&socket.id).unwrap().clone();
    //at this point we can be sure that a song was actually sent
    db.votes.vote(song.unwrap(), username);
    let votes = db.votes.get_all();
    println!("{:?}", votes);
    let _ = io.emit("votes", &votes).await;
}

async fn onsearch(
    socket: SocketRef,
    State(db): State<Arc<Mutex<Db>>>,
    Data(search): Data<SongSearch>,
) {
    let db = db.lock().await;
    match &db.client {
        None => {
            let _ = socket.emit("search", "client not ready");
        }
        Some(client) => {
            let response = client
                .clone()
                .search(
                    search.searchstring,
                    &[Item::Album, Item::Artist, Item::Track],
                )
                .get()
                .await;
            match response {
                Ok(responsecontent) => {
                    let result = responsecontent.tracks;
                    //This might need conversion into a Vec<Songs>
                    let _ = socket.emit("search", &result);
                }
                Err(err) => {
                    let _ = socket.emit("search", &err);
                }
            };
            // let _ = socket.emit("search", &result);
        }
    }
}

pub async fn auth_middleware(
    socket: SocketRef,
    Data(data): Data<Value>,
    State(db): State<Arc<Mutex<Db>>>,
) -> Result<(), AuthError> {
    // info!(?data, "Socket auth");
    let binding = match data.as_map() {
        Some(map) => match map.first() {
            Some(data) => data.1.clone(),
            None => {
                info!(?map, "Socket rejected");
                return Err(AuthError {
                    message: "Empty Token not allowed".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                });
            }
        },
        None => {
            info!(?data, "Socket rejected");
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
            info!(?data, "Socket rejected");
            return Err(AuthError {
                message: "Error decoding token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    }
    Ok(())
}
