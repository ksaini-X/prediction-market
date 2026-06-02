use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CustomError {
    MarketNotFound,
    UserNotFound,
    InsufficientUserBalance,
    InsufficientUserLockedBalance,
    InvalidHoldingsForMerge,
}
