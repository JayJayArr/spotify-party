use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use tokio::sync::Mutex;

use crate::{Db, PEAK_ALLOC};

pub async fn health_handler(State(db): State<Arc<Mutex<Db>>>) -> Result<Json<Health>, StatusCode> {
    let current_mem = PEAK_ALLOC.current_usage_as_kb();
    let db = db.lock().await;
    if db.is_healthy() {
        Ok(Json(Health {
            mem_in_kb: current_mem,
            dbhealthy: true,
        }))
    } else {
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Health {
    mem_in_kb: f32,
    dbhealthy: bool,
}
