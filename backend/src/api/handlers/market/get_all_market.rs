use std::sync::{Arc, Mutex};

use axum::{Json, extract::State};
use tokio::sync::oneshot;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::engine::Market,
};

pub async fn get_all_markets(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Market>>, CustomError> {
    let (s, r) = oneshot::channel::<Vec<Market>>();
    state
        .lock()
        .unwrap()
        .tx
        .send(EngineMessage::GetAllMarkets { reply: s })
        .await
        .map_err(|_| CustomError::EngineDown)?;
    let markets = r.await.unwrap();
    Ok(Json(markets))
}
