use std::sync::{Arc, Mutex};

use axum::{Json, extract::State};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

#[derive(Debug)]
pub struct DeleteMarketData {
    pub market_id: Uuid,
}

pub async fn delete_market(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<DeleteMarketData>,
) -> Result<Json<Market>, CustomError> {
    let (s, r) = oneshot::channel::<Market>();

    state
        .lock()
        .unwrap()
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
