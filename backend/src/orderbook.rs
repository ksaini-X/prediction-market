use std::collections::BTreeMap;

use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    order_id: Uuid,
    user_id: Uuid,
    price: Decimal,
    quantity: Decimal,
    filled_quantity: Decimal,
    order_type: OrderType,
    order_side: OrderSide,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum OrderType {
    Buy,
    Sell,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum OrderSide {
    Yes,
    No,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Orderbook {
    pub bids: BTreeMap<Decimal, Vec<Order>>,
    pub asks: BTreeMap<Decimal, Vec<Order>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlaceOrder {
    user_id: Uuid,
    price: Decimal,
    quantity: Decimal,
    order_type: OrderType,
    order_side: OrderSide,
}

pub struct Fill {
    pub maker_order_id: Uuid,
    pub taker_order_id: Uuid,
    pub quantity: Decimal,
    pub price: Decimal,
}

impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn place_order(&mut self, place_order_data: PlaceOrder) {
        let mut order = Order {
            filled_quantity: dec!(0),
            order_id: Uuid::new_v4(),
            order_side: place_order_data.order_side,
            order_type: place_order_data.order_type,
            price: place_order_data.price,
            quantity: place_order_data.quantity,
            user_id: place_order_data.user_id,
        };

        match order.order_type {
            OrderType::Buy => {
                self.match_asks(&mut order);
            }
            OrderType::Sell => {
                self.match_bids(&mut order);
            }
        };
    }

    pub fn match_asks(&mut self, order: &mut Order) -> (Decimal, Vec<Fill>) {
        //Incoming order is willing to Buy
        // {Bid, price:98, q:10} (Usually the Bid order price would be less that the best ask)
        // Asks are sorted ascedingly (no need to reverse)
        // Price Qyt
        // 99   10
        // 100  11
        // 101  1
        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();
        for (price, asks) in self.asks.iter_mut() {
            if &order.price < price {
                break;
            }
            if executed_quantity < order.quantity {
                for ask in asks.iter_mut() {
                    let left_quantity = order.quantity - executed_quantity;
                    let filled_quantity = std::cmp::min(left_quantity, ask.quantity);
                    ask.filled_quantity += filled_quantity;
                    order.filled_quantity += filled_quantity;
                    executed_quantity += filled_quantity;
                    fills.push(Fill {
                        maker_order_id: order.order_id,
                        taker_order_id: ask.order_id,
                        quantity: filled_quantity,
                        price: ask.price,
                    });
                }
            }
            asks.retain(|ask| ask.filled_quantity < ask.quantity);
        }
        (executed_quantity, fills)
    }
    pub fn match_bids(&mut self, order: &mut Order) {}
}

#[test]
pub fn setup_orderbook() {
    let book = Orderbook::new();
    assert_eq!(book.asks.len(), 0);
    assert_eq!(book.bids.len(), 0);
}
