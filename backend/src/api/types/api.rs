use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{
    engine::{Holdings, User},
    orderbook::Fill,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCreated {
    pub user_id: Uuid,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUser {
    pub user_id: Uuid,
    pub balance: Decimal,
    pub locked_balance: Decimal,
    pub positions: HashMap<Uuid, Holdings>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AllUsers {
    pub users: HashMap<Uuid, User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPlaced {
    pub order_id: Uuid,
    pub executed_quantity: Decimal,
    pub fills: Vec<Fill>,
}
