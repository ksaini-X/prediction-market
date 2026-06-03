use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

#[derive(Debug, Deserialize)]
pub struct DeleteMarketData {
    pub market_id: Uuid,
}

pub async fn delete_market(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DeleteMarketData>,
) -> Result<Json<Market>, CustomError> {
    let (s, r) = oneshot::channel::<Market>();

    state
        .tx
        .send(EngineMessage::DeleteMarket {
            market_id: payload.market_id,
            reply: s,
        })
        .await
        .map_err(|_| CustomError::EngineDown)?;

    let market = r.await.unwrap();
    Ok(Json(market))
}
