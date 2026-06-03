use tokio::sync::oneshot;
use uuid::Uuid;

use crate::{
    api::types::api::{AllUsers, OrderPlaced, UserCreated},
    error::CustomError,
    types::{
        engine::{Market, User},
        orderbook::OrderAction,
    },
};

pub enum EngineMessage {
    GetMarket {
        market_id: Uuid,
        reply: oneshot::Sender<Market>,
    },
    ResolveMarket {
        market_id: Uuid,
        outcome: u8,
        reply: oneshot::Sender<Market>,
    },
    GetAllMarkets {
        reply: oneshot::Sender<Vec<Market>>,
    },
    DeleteMarket {
        market_id: Uuid,
        reply: oneshot::Sender<Market>,
    },
    GetUser {
        user_id: Uuid,
        reply: oneshot::Sender<User>,
    },
    CreateUser {
        reply: oneshot::Sender<UserCreated>,
    },
    GetAllUsers {
        reply: oneshot::Sender<AllUsers>,
    },
    PlaceOrder {
        market_id: Uuid,
        user_id: Uuid,
        action: OrderAction,
        reply: oneshot::Sender<Result<OrderPlaced, CustomError>>,
    },
    CreateMarket {
        title: String,
        resolution_time: i64,
        reply: oneshot::Sender<Market>,
    },
}
