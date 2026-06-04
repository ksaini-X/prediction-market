use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    AppState, api::types::engine::EngineMessage, error::CustomError, types::orderbook::Order,
};

#[derive(Serialize, Debug, Deserialize)]
pub struct GetAllOrders {
    pub user_id: Uuid,
}
pub async fn get_all_orders_for_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetAllOrders>,
) -> Result<Json<Vec<Order>>, CustomError> {
    let (s, r) = oneshot::channel::<Vec<Order>>();
    state
        .tx
        .send(EngineMessage::GetAllOrdersForUser {
            user_id: payload.user_id,
            reply: s,
        })
        .await
        .unwrap();

    let orders = r.await.unwrap();
    Ok(Json(orders))
}
