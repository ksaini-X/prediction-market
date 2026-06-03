use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    api::types::api::{OrderPlaced, UserCreated},
    error::CustomError,
    types::orderbook::OrderAction,
};

pub enum EngineMessage {
    CreateUser {
        reply: oneshot::Sender<UserCreated>,
    },
    PlaceOrder {
        market_id: Uuid,
        user_id: Uuid,
        action: OrderAction,
        reply: oneshot::Sender<Result<OrderPlaced, CustomError>>,
    },
}
