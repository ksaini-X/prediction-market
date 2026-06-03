use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::{Mutex, mpsc::Sender};

use crate::api::{
    create_user::create_user,
    types::{api::UserCreated, engine::EngineMessage},
};
pub mod api;
pub mod engine;
pub mod error;
pub mod orderbook;
pub mod tests;
pub mod types;

pub struct AppState {
    tx: Sender<EngineMessage>,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<EngineMessage>(100);

    tokio::spawn(async move {
        let mut engine = engine::Engine::new();
        while let Some(msg) = rx.recv().await {
            match msg {
                EngineMessage::PlaceOrder {
                    market_id,
                    user_id,
                    action,
                    reply,
                } => {
                    let r = engine.place_order(market_id, user_id, action);
                }
                EngineMessage::CreateUser { reply } => {
                    let user_id = engine.add_user();
                    reply.send(UserCreated { user_id }).unwrap();
                }
            }
        }
    });
    let shared_state = Arc::new(Mutex::new(AppState { tx }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/", get(create_user))
        .with_state(shared_state);

    axum::serve(listener, router).await.unwrap()
}
