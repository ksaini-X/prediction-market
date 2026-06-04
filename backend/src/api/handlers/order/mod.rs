use std::sync::Arc;

use axum::{
    Router,
    routing::{Route, get},
};

use crate::AppState;

pub mod cancel_order;
pub mod get_all_orders_for_user;
pub mod place_order;

pub fn order_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_all_orders_for_user::get_all_orders_for_user))
}
