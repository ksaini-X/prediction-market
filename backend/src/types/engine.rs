use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::orderbook::Orderbook;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Market {
    pub title: String,
    pub resolution_time: i64,
    pub resolved: bool,
    pub market_id: Uuid,
    pub resolved_outcome: Option<u8>,
    pub orderbook: Orderbook,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Holdings {
    pub yes: Decimal,
    pub no: Decimal,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub balance: Decimal,
    pub locked_balance: Decimal,
    pub holdings: HashMap<Uuid, Holdings>,
}
