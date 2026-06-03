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
    InvalidResolutionOutcome,
    InvalidResolutionTime,
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
            CustomError::InvalidResolutionOutcome => {
                (StatusCode::BAD_REQUEST, "Outcome should be 0(NO) or 1(YES)")
            }
            CustomError::InvalidResolutionTime => (
                StatusCode::BAD_REQUEST,
                "Current time is less than resolution time",
            ),
        };
        (status, msg).into_response()
    }
}
