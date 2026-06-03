use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::AppState;

pub mod create_user;
pub mod get_all_users;
pub mod get_user;

pub fn user_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_all_users::get_all_users))
        .route("/user", get(get_user::get_user))
        .route("/user", post(create_user::create_user))
}
