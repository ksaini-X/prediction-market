use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    AppState,
    api::types::engine::EngineMessage,
    error::CustomError,
    types::orderbook::{Fill, OrderAction},
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub struct PlaceOrderData {
    order_action: OrderAction,
    user_id: Uuid,
    market_id: Uuid,
}
pub async fn place_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<PlaceOrderData>,
) -> Result<Json<(Uuid, Vec<Fill>)>, CustomError> {
    let (s, r) = oneshot::channel::<(Uuid, Vec<Fill>)>();

    state
        .tx
        .send(EngineMessage::PlaceOrder {
            market_id: payload.market_id,
            user_id: payload.user_id,
            order_action: payload.order_action,
            reply: s,
        })
        .await
        .unwrap();

    let res = r.await.unwrap();
    Ok(Json(res))
}
