use std::sync::{Arc, Mutex};

use axum::{Json, extract::State};
use tokio::sync::oneshot;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

#[derive(Debug)]
pub struct CreateMarketData {
    pub title: String,
    pub resolution_time: i64,
}

pub async fn create_market(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CreateMarketData>,
) -> Result<Json<Market>, CustomError> {
    let (s, r) = oneshot::channel::<Market>();

    state
        .lock()
        .unwrap()
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
