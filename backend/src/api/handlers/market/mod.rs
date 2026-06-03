use crate::AppState;
use axum::{
    Router,
    routing::{delete, get, post},
};
use std::sync::Arc;

pub mod create_market;
pub mod delete_market;
pub mod get_all_markets;
pub mod get_market;
pub mod resolve_market;

pub fn market_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/markets", get(get_all_markets::get_all_markets))
        .route("/market", post(create_market::create_market))
        .route("/market", delete(delete_market::delete_market))
        .route("/market/resolve", post(resolve_market::resolve_market))
}
