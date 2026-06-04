use std::sync::Arc;

use axum::{Json, extract::State};
use serde::Serialize;
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{AppState, error::CustomError, types::orderbook::Order};

#[derive(Serialize)]
pub struct CancelOrder {
    user_id: Uuid,
    order_id: Uuid,
    market_id: Uuid,
}

pub async fn cancel_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CancelOrder>,
) -> Result<Json<Order>, CustomError> {
    let (s, r) = oneshot::channel::<Order>();
    state.tx
}
