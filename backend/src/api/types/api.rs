use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::orderbook::Fill;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreated {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPlaced {
    pub order_id: Uuid,
    pub executed_quantity: Decimal,
    pub fills: Vec<Fill>,
}
