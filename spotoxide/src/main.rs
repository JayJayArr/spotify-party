use std::{collections::HashMap, sync::Arc};

use auth::signin_handler;
use axum::routing::{get, post};
use db::Db;
use dotenvy::dotenv;
use handler::*;
use health::health_handler;
use iohandler::{auth_middleware, on_connect};
use peak_alloc::PeakAlloc;
use rnglib::{Language, RNG};
use socketioxide::{SocketIoBuilder, handler::ConnectHandler};
use song_queue::SongQueue;
use spotify_rs::{AuthCodeClient, AuthCodeFlow, RedirectUrl};
use tokio::{signal, sync::Mutex};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_util::task::TaskTracker;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use user::Usernames;
use votes::Votes;

mod app;
mod auth;
mod db;
mod handler;
mod health;
mod iohandler;
mod song;
mod song_queue;
mod user;
mod votes;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    dotenv().ok();
    let mut store = HashMap::new();
    let client_id =
        std::env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID must be specified");
    info!(?client_id, "Spotify Client Id");
    store.insert("client_id", client_id.clone());
    let client_secret =
        std::env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET must be specified");
    info!(?client_secret, "Spotify Client Secret");
    store.insert("client_secret", client_secret.clone());

    //setup components
    let rng = RNG::from(&Language::Fantasy);
    let queue = SongQueue::new();
    let usernames = Usernames::new();
    let votes = Votes::new();

    //Start the authorization code flow
    let redirect_uri = RedirectUrl::new("http://localhost:3000/redirect".to_owned())?;
    let auto_refresh = true;
    let scopes = vec![
        "user-library-read",
        "playlist-read-private",
        "user-read-currently-playing",
        "user-read-playback-state",
        "user-modify-playback-state",
    ];
    let auth_code_flow = AuthCodeFlow::new(client_id, client_secret, scopes);
    let (mut client, url) = AuthCodeClient::new(auth_code_flow, redirect_uri, auto_refresh);
    client.auto_refresh = true;
    let redirecturlstring = url.to_string();

    let db = Db {
        users: usernames,
        votes,
        rng,
        queue,
        client_unauth: client,
        client: None,
    };
    //wrap in an Arc, Mutex
    let dbarc = Arc::new(Mutex::new(db));

    //create io layer
    let (iolayer, io) = SocketIoBuilder::new()
        .with_state(dbarc.clone())
        .build_layer();

    io.ns("/", on_connect.with(auth_middleware));

    let mut sched = JobScheduler::new().await?;
    //create a cron job to update the queue
    let crondbhandle = dbarc.clone();
    let croniohandle = io.clone();
    sched
        .add(Job::new_async("every 10 seconds", move |_uuid, _l| {
            Box::pin({
                let dbhandle = crondbhandle.clone();
                let iohandle = croniohandle.clone();
                async move {
                    info!("Updating queue");
                    let db = &mut dbhandle.lock().await;
                    match &mut db.client {
                        None => {}
                        Some(client) => {
                            let currently_playing = client.get_user_queue().await.unwrap();
                            // info!(?currently_playing, "currently_playing");
                            db.queue = currently_playing.into();
                            let _ = iohandle.emit("songs", &db.queue.get()).await;
                        }
                    }
                }
            })
        })?)
        .await?;
    //create a second cron job
    let crondbhandle = dbarc.clone();
    let croniohandle = io.clone();
    sched
        .add(Job::new_async("every 2 minutes", move |_uuid, _l| {
            Box::pin({
                let dbhandle = crondbhandle.clone();
                let iohandle = croniohandle.clone();
                async move {
                    info!("Vote cycle");
                    let db = &mut dbhandle.lock().await;
                    if let Some(songid) = &db.votes.get_most_popular() {
                        let songid = (songid.clone()).clone();
                        match &mut db.client {
                            None => {
                                info!("Client unauthorized");
                            }
                            Some(client) => {
                                info!("Client active");
                                client.add_item_to_queue(songid);
                                let _ = iohandle.emit("songs", &db.votes.get_all()).await;
                            }
                        }
                    }
                }
            })
        })?)
        .await?;

    let tracker = TaskTracker::new();
    sched.start().await.expect("could not start cron scheduler");
    tracker.spawn(async move {
        let app = axum::Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/signin", post(signin_handler))
            .route("/login", get(|| async { redirecturlstring }))
            .route("/redirect", get(redirect_handler))
            .route("/health", get(health_handler))
            .with_state(dbarc.clone())
            .layer(iolayer)
            .layer(CorsLayer::permissive());

        info!("Starting server");

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("could not bind to port");
        axum::serve(listener, app)
            .with_graceful_shutdown(app::shutdown_signal())
            .await
            .expect("Could not start axum server");
    });
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("graceful shutdown initiated, shutting cron scheduler down");
            let _ = sched.shutdown().await;
            tracker.close();
            info!("tracker closing");
            tracker.wait().await;
            info!("good bye");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        }
    }

    Ok(())
}
