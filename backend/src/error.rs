use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum CustomError {
    EngineDown,

    MarketNotFound,
    UserNotFound,
    InsufficientUserBalance,
    InsufficientUserLockedBalance,
    InvalidHoldingsForMerge,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = match self {
            CustomError::EngineDown => (StatusCode::SERVICE_UNAVAILABLE, "Engine is down"),
            CustomError::MarketNotFound => (StatusCode::NOT_FOUND, "Market not found"),
            CustomError::UserNotFound => (StatusCode::NOT_FOUND, "User not found"),
            CustomError::InsufficientUserBalance => {
                (StatusCode::BAD_REQUEST, "Insufficient balance")
            }
            CustomError::InsufficientUserLockedBalance => {
                (StatusCode::BAD_REQUEST, "Insufficient locked balance")
            }
            CustomError::InvalidHoldingsForMerge => {
                (StatusCode::BAD_REQUEST, "Invalid holdings for merge")
            }
        };
        (status, msg).into_response()
    }
}
