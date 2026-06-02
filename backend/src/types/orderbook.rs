use std::{cmp, collections::BTreeMap};

use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub price: Decimal,
    pub quantity: Decimal,
    pub filled_quantity: Decimal,
    pub order_type: OrderType,
    pub order_side: Option<OrderSide>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub enum OrderType {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Copy)]
pub enum OrderSide {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PlaceOrder {
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_type: OrderType,
    pub order_side: Option<OrderSide>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum OrderAction {
    PlaceOrder(PlaceOrder),
    Split(Decimal),
    Merge(Decimal),
}
pub struct Fill {
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub quantity: Decimal,
    pub price: Decimal,
}
