use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
};
use serde::Serialize;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    AppState, api::types::engine::EngineMessage, engine::Engine, error::CustomError,
    types::engine::Market,
};

#[derive(Serialize)]
pub struct GetMarketData {
    market_id: Uuid,
}
pub async fn get_market(
    Json(payload): Json<GetMarketData>,
    State(state): State<Arc<AppState>>,
) -> Result<Market, CustomError> {
    let (s, r) = oneshot::channel::<Market>();
    state
        .tx
        .send(EngineMessage::GetMarket {
            market_id: payload.market_id,
            reply: s,
        })
        .await
        .unwrap();
    let res = r.await.unwrap();
    Ok(res)
}
