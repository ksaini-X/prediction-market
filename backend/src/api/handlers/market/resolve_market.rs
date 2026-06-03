use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};
#[derive(Debug, Deserialize)]

pub struct ResolveData {
    pub market_id: Uuid,
    pub outcome: u8,
}

pub async fn resolve_market(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResolveData>,
) -> Result<Json<Market>, CustomError> {
    let (s, r) = oneshot::channel::<Market>();
    state
        .tx
        .send(EngineMessage::ResolveMarket {
            market_id: payload.market_id,
            outcome: payload.outcome,
            reply: s,
        })
        .await
        .map_err(|_| CustomError::EngineDown)?;

    let res = r.await.map_err(|_| CustomError::EngineDown)?;
    Ok(Json(res))
}
