use std::sync::{Arc, Mutex};

use axum::{Json, extract::State};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{AppState, api::types::engine::EngineMessage, error::CustomError};

pub struct ResolveData {
    pub market_id: Uuid,
    pub outcome: u8,
}

pub async fn resolve_market(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ResolveData>,
) -> Result<(), CustomError> {
    let (s, r) = oneshot::channel::<()>();
    state
        .lock()
        .unwrap()
        .tx
        .send(EngineMessage::ResolveMarket {
            market_id: payload.market_id,
            outcome: payload.outcome,
            reply: s,
        })
        .await
        .map_err(|_| CustomError::EngineDown);

    let res = r.await.unwrap();
    Ok(res)
}
