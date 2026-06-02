use crate::orderbook::*;
use crate::{orderbook::*, types::orderbook::*};
use rust_decimal::dec;

use uuid::Uuid;
#[test]
pub fn add_orders_to_orderbook() {
    let mut book = Orderbook::new();

    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(100),
        quantity: dec!(10),
        order_type: OrderType::Buy,
        order_side: Some(OrderSide::Yes),
    });
    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(101),
        quantity: dec!(10),
        order_type: OrderType::Buy,
        order_side: Some(OrderSide::Yes),
    });
    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(102),
        quantity: dec!(10),
        order_type: OrderType::Sell,
        order_side: Some(OrderSide::Yes),
    });

    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(110),
        quantity: dec!(10),
        order_type: OrderType::Sell,
        order_side: Some(OrderSide::Yes),
    });
    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(109),
        quantity: dec!(10),
        order_type: OrderType::Sell,
        order_side: Some(OrderSide::Yes),
    });
    book.place_order(PlaceOrder {
        user_id: Uuid::new_v4(),
        price: dec!(108),
        quantity: dec!(10),
        order_type: OrderType::Sell,
        order_side: Some(OrderSide::Yes),
    });

    assert_eq!(book.bids.len(), 2);
    assert_eq!(book.asks.len(), 4);
}
