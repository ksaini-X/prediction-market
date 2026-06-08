use crate::{error::CustomError, types::orderbook::*};
use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Orderbook {
    pub bids: BTreeMap<Decimal, Vec<Order>>,
    pub asks: BTreeMap<Decimal, Vec<Order>>,
}
impl Orderbook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn place_order(
        &mut self,
        place_order_data: PlaceOrder,
        user_id: Uuid,
    ) -> (Uuid, Decimal, Vec<Fill>) {
        let mut order = Order {
            filled_quantity: dec!(0),
            order_id: Uuid::new_v4(),
            order_side: place_order_data.order_side,
            order_type: place_order_data.order_type,
            price: place_order_data.price,
            quantity: place_order_data.quantity,
            user_id,
        };

        let mut executed_quantity: Decimal = dec!(0);
        let mut fills: Vec<Fill> = Vec::new();

        if order.order_side == Some(OrderSide::No) {
            order.price = dec!(1) - order.price;
            if order.order_type == OrderType::Buy {
                order.order_type = OrderType::Sell
            } else if order.order_type == OrderType::Sell {
                order.order_type = OrderType::Buy
            }
        }

        match order.order_type {
            OrderType::Buy => {
                (executed_quantity, fills) = self.match_asks(&mut order);
                if executed_quantity < order.quantity {
                    order.filled_quantity = executed_quantity;
                    self.bids
                        .entry(order.price)
                        .and_modify(|bids| bids.push(order.clone()))
                        .or_insert(vec![order.clone()]);
                }
            }
            OrderType::Sell => {
                (executed_quantity, fills) = self.match_bids(&mut order);
                order.filled_quantity = executed_quantity;
                if executed_quantity < order.quantity {
                    self.asks
                        .entry(order.price)
                        .and_modify(|asks| asks.push(order.clone()))
                        .or_insert(vec![order.clone()]);
                }
            }
        };
        (order.order_id, executed_quantity, fills)
    }
    pub fn cancel_order(&mut self, order_id: Uuid) -> Result<Order, CustomError> {
        for (price, asks) in self.asks.iter_mut() {
            if let Some(order) = asks.iter().find(|o| o.order_id == order_id) {
                asks.retain(|o| o.order_id != order_id);
                return Ok(order.clone());
            } else {
                return Err(CustomError::OrderNotFound);
            }
        }
        for (price, asks) in self.bids.iter_mut() {
            if let Some(order) = asks.iter().find(|o| o.order_id == order_id) {
                asks.retain(|o| o.order_id != order_id);
                return Ok(order.clone());
            } else {
                return Err(CustomError::OrderNotFound);
            }
        }
    }
    pub fn match_asks(&mut self, order: &mut Order) -> (Decimal, Vec<Fill>) {
        //Incoming order is willing to Buy
        // {Bid, price:98, q:10} (Usually the Bid order price would be less that the best ask)
        // Asks are stored in asscending order (no need to reverse)
        // Price Qyt
        // 99   10
        // 100  11
        // 101  1
        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();
        for (price, asks) in self.asks.iter_mut() {
            // the best ask here is at 99
            // if the incoming order is willing Buy at <99 (say 98), return
            if &order.price < price {
                break;
            }
            if executed_quantity < order.quantity {
                for ask in asks.iter_mut() {
                    let left_quantity = order.quantity - order.filled_quantity;
                    let filled_quantity =
                        std::cmp::min(left_quantity, ask.quantity - ask.filled_quantity);
                    ask.filled_quantity += filled_quantity;
                    order.filled_quantity += filled_quantity;
                    executed_quantity += filled_quantity;
                    fills.push(Fill {
                        taker_order_id: order.order_id,
                        maker_order_id: ask.order_id,
                        quantity: filled_quantity,
                        price: ask.price,
                    });
                }
            }
            asks.retain(|ask| ask.filled_quantity < ask.quantity);
        }
        self.asks.retain(|_p, ask| !ask.is_empty());

        (executed_quantity, fills)
    }
    pub fn match_bids(&mut self, order: &mut Order) -> (Decimal, Vec<Fill>) {
        // Incoming order is willing to Sell
        // Need to rev the bids as the best bid should be the largest price
        // Bids (after rev)
        // 100   10
        // 88    20
        let mut executed_quantity = dec!(0);
        let mut fills = Vec::<Fill>::new();

        for (price, bids) in self.bids.iter_mut().rev() {
            if &order.price > price {
                // Incoming order is willing to sell
                // if incoming order price is greater(say 101) than the best bid 100, return
                break;
            }
            if executed_quantity < order.quantity {
                for bid in bids.iter_mut() {
                    let left_quantity = order.quantity - order.filled_quantity;
                    let filled_quantity =
                        std::cmp::min(left_quantity, bid.quantity - bid.filled_quantity);
                    executed_quantity += filled_quantity;
                    bid.filled_quantity += filled_quantity;
                    order.filled_quantity += filled_quantity;
                    fills.push(Fill {
                        maker_order_id: bid.order_id,
                        taker_order_id: order.order_id,
                        quantity: filled_quantity,
                        price: bid.price,
                    });
                }
            }
            bids.retain(|bid| bid.filled_quantity < bid.quantity);
        }
        self.bids.retain(|_p, bid| !bid.is_empty());

        (executed_quantity, fills)
    }

    pub fn get_all_orders_for_user(&self, user_id: Uuid) -> Vec<Order> {
        let mut orders: Vec<Order> = self
            .bids
            .values()
            .flatten()
            .filter(|o| o.user_id == user_id)
            .cloned()
            .collect();

        let asks: Vec<Order> = self
            .asks
            .values()
            .flatten()
            .filter(|o| o.user_id == user_id)
            .cloned()
            .collect();

        orders.extend(asks);
        orders
    }
}
