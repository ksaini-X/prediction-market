use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Deserialize;
use tokio::sync::{Mutex, oneshot};

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

#[derive(Debug, Deserialize)]
pub struct CreateMarketData {
    pub title: String,
    pub resolution_time: i64,
}

pub async fn create_market(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateMarketData>,
) -> Result<Json<Market>, CustomError> {
    let (s, r) = oneshot::channel::<Market>();

    state
        .tx
        .send(EngineMessage::CreateMarket {
            title: payload.title,
            resolution_time: payload.resolution_time,
            reply: s,
        })
        .await
        .map_err(|_| CustomError::EngineDown)?;

    let market = r.await.unwrap();
    Ok(Json(market))
}
