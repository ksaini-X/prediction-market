use crate::orderbook::*;
use rust_decimal::dec;
use uuid::Uuid;
#[test]
pub fn setup_orderbook() {
    let book = Orderbook::new();
    assert_eq!(book.asks.len(), 0);
    assert_eq!(book.bids.len(), 0);
}
