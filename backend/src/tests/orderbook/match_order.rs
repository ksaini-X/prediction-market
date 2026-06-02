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
        price: dec!(101),
        quantity: dec!(100),
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

    println!("{:?}", book);

    assert_eq!(book.bids.get(&dec!(101)).unwrap().len(), 0);
    assert_eq!(
        book.asks.get(&dec!(101)).unwrap()[0].filled_quantity,
        dec!(10)
    );
    assert_eq!(
        book.asks.get(&dec!(101)).unwrap()[0].quantity
            - book.asks.get(&dec!(101)).unwrap()[0].filled_quantity,
        dec!(90)
    );
}
