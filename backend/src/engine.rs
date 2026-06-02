use crate::{error::CustomError, types::orderbook::OrderType};
use rust_decimal::dec;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::{
    orderbook::Orderbook,
    types::{
        engine::{Market, User},
        orderbook::PlaceOrder,
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Engine {
    pub markets: HashMap<Uuid, Market>,
    pub users: HashMap<Uuid, User>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            markets: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self) -> Uuid {
        let user_id = Uuid::new_v4();
        self.users.insert(
            user_id,
            User {
                user_id,
                balance: dec!(1000),
            },
        );
        user_id
    }

    pub fn create_market(&mut self, resolution_time: i64, title: String) -> Uuid {
        let market_id = Uuid::new_v4();
        let market = Market {
            market_id: Uuid::new_v4(),
            orderbook: Orderbook {
                bids: BTreeMap::new(),
                asks: BTreeMap::new(),
            },
            resolution_time,
            resolved: false,
            title,
        };
        self.markets.insert(market_id, market);
        market_id
    }

    pub fn place_order(
        &mut self,
        market_id: Uuid,
        place_order_data: PlaceOrder,
    ) -> Result<(), CustomError> {
        let market = self.markets.get_mut(&market_id);
        match market {
            None => Err(CustomError::MarketNotFound),
            Some(market) => {
                market.orderbook.place_order(place_order_data);
                Ok(())
            }
        }
    }
}
