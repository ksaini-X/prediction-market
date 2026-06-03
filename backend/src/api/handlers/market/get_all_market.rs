use std::sync::Arc;

use axum::{Json, extract::State};
use tokio::sync::{Mutex, oneshot};

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

pub async fn get_all_markets(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Market>>, CustomError> {
    let (s, r) = oneshot::channel::<Vec<Market>>();
    state
        .tx
        .send(EngineMessage::GetAllMarkets { reply: s })
        .await
        .map_err(|_| CustomError::EngineDown)?;
    let markets = r.await.unwrap();
    Ok(Json(markets))
}
