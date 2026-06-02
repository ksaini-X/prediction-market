use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{orderbook::Orderbook, types::orderbook::PlaceOrder};

#[derive(Serialize, Deserialize, Clone)]
pub struct Market {
    pub title: String,
    pub resolution_time: i64,
    pub resolved: bool,
    pub market_id: Uuid,
    pub orderbook: Orderbook,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Holdings {
    pub yes: Decimal,
    pub no: Decimal,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub balance: Decimal,
    pub locked_balance: Decimal,
    pub positions: HashMap<Uuid, Holdings>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MarketAction {
    PlaceOrder(PlaceOrder),
    Split { quantity: Decimal },
    Merge { quantity: Decimal },
}
