use crate::{
    error::CustomError,
    types::{
        engine::Holdings,
        orderbook::{Fill, OrderAction},
    },
};
use rust_decimal::{Decimal, dec};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::{
    orderbook::Orderbook,
    types::engine::{Market, User},
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
            market_id,
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
        let user = self.users.get(&user_id).ok_or(CustomError::UserNotFound)?;
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
            .ok_or(CustomError::UserNotFound)?;
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
            .ok_or(CustomError::UserNotFound)?;

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
            .ok_or(CustomError::UserNotFound)?;
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
        user_id: Uuid,
        order_action: OrderAction,
    ) -> Result<(), CustomError> {
        match order_action {
            OrderAction::Split(amount) => {
                self.deduct_user_balance(user_id, amount)?;
                let user_position = self
                    .users
                    .get_mut(&user_id)
                    .unwrap()
                    .positions
                    .entry(market_id)
                    .or_insert(Holdings {
                        no: dec!(0),
                        yes: dec!(0),
                    });
                user_position.no += amount;
                user_position.yes += amount;
                Ok(())
            }
            OrderAction::Merge(amount) => {
                let user_position = self
                    .users
                    .get_mut(&user_id)
                    .unwrap()
                    .positions
                    .get_mut(&market_id)
                    .unwrap();

                if user_position.no < amount || user_position.yes < amount {
                    return Err(CustomError::InvalidHoldingsForMerge);
                } else {
                    user_position.no -= amount;
                    user_position.yes -= amount;
                    self.deposit_user_balance(user_id, amount)?;
                }
                Ok(())
            }
            OrderAction::PlaceOrder(place_order_data) => {
                let amount = place_order_data.price * place_order_data.quantity;
                self.deduct_user_balance(user_id, amount)?;

                let market = self
                    .markets
                    .get_mut(&market_id)
                    .ok_or(CustomError::MarketNotFound)?;

                let (order_id, executed_quantity, fills) =
                    market.orderbook.place_order(place_order_data, user_id);
                if executed_quantity > dec!(0) {
                    let user_positions = self
                        .users
                        .get_mut(&user_id)
                        .unwrap()
                        .positions
                        .entry(market_id)
                        .or_insert(Holdings {
                            yes: dec!(0),
                            no: dec!(0),
                        });

                    match place_order_data.order_side {
                        Some(crate::types::orderbook::OrderSide::No) => {
                            user_positions.no += executed_quantity;
                        }
                        Some(crate::types::orderbook::OrderSide::Yes) => {
                            user_positions.yes += executed_quantity;
                        }
                        None => (),
                    }
                }
                Ok(())
            }
        }
    }
}
