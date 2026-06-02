use crate::{error::CustomError, types::orderbook::Fill};
use rust_decimal::{Decimal, dec};
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
                locked_balance: dec!(0),
                positions: HashMap::new(),
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

    pub fn check_user_balance(&self, user_id: Uuid, amount: Decimal) -> Result<(), CustomError> {
        let user = self
            .users
            .get(&user_id)
            .ok_or(CustomError::UserNotFound)
            .unwrap();
        if user.balance < amount {
            return Err(CustomError::InsufficientUserBalance);
        }
        Ok(())
    }
    pub fn deduct_user_balance(
        &mut self,
        user_id: Uuid,
        amount: Decimal,
    ) -> Result<(), CustomError> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(CustomError::UserNotFound)
            .unwrap();
        if user.balance < amount {
            return Err(CustomError::InsufficientUserBalance);
        }
        user.balance -= amount;
        user.locked_balance += amount;
        Ok(())
    }
    pub fn deposit_user_balance(
        &mut self,
        user_id: Uuid,
        amount: Decimal,
    ) -> Result<(), CustomError> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(CustomError::UserNotFound)
            .unwrap();

        user.balance += amount;
        Ok(())
    }
    pub fn unlock_user_balance(
        &mut self,
        user_id: Uuid,
        amount: Decimal,
    ) -> Result<(), CustomError> {
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(CustomError::UserNotFound)
            .unwrap();
        if user.locked_balance < amount {
            return Err(CustomError::InsufficientUserLockedBalance);
        }
        user.balance += amount;
        user.locked_balance -= amount;
        Ok(())
    }
    pub fn place_order(
        &mut self,
        market_id: Uuid,
        place_order_data: PlaceOrder,
    ) -> Result<(Uuid, Decimal, Vec<Fill>), CustomError> {
        let amount = place_order_data.price * place_order_data.quantity;

        match place_order_data.order_action {
            Some(crate::types::orderbook::OrderAction::Split(amount)) => {
                self.check_user_balance(place_order_data.user_id, amount)?;
                self.deduct_user_balance(place_order_data.user_id, amount)?;
                let position = self
                    .users
                    .get_mut(&place_order_data.user_id)
                    .unwrap()
                    .positions
                    .get_mut(&market_id)
                    .unwrap();
                position.no += amount;
                position.yes += amount;
            }
            Some(crate::types::orderbook::OrderAction::Merge(amount)) => {
                let user_position = self
                    .users
                    .get_mut(&place_order_data.user_id)
                    .unwrap()
                    .positions
                    .get_mut(&market_id)
                    .unwrap();
                if user_position.no < amount || user_position.yes < amount {
                    return Err(CustomError::InvalidHoldingsForMerge);
                } else {
                    user_position.no -= amount;
                    user_position.yes -= amount;
                    self.deposit_user_balance(place_order_data.user_id, amount)?;
                }
            }
            None => {}
        }
        self.check_user_balance(place_order_data.user_id, amount)?;
        self.deduct_user_balance(place_order_data.user_id, amount)?;

        let market = self
            .markets
            .get_mut(&market_id)
            .ok_or(CustomError::MarketNotFound)
            .unwrap();

        let (order_id, executed_quantity, fills) = market.orderbook.place_order(place_order_data);
        if executed_quantity > dec!(0) {
            let user_positions = self
                .users
                .get_mut(&place_order_data.user_id)
                .unwrap()
                .positions
                .get_mut(&market_id)
                .unwrap();

            match place_order_data.order_side {
                Some(crate::types::orderbook::OrderSide::No) => {
                    user_positions.no += executed_quantity;
                }
                Some(crate::types::orderbook::OrderSide::Yes) => {
                    user_positions.yes += executed_quantity;
                }
                None => {}
            }
        }
        return Ok((order_id, executed_quantity, fills));
    }
}
