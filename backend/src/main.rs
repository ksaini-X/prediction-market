use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::{Mutex, mpsc::Sender};

use crate::api::{
    handlers::user::{create_user::create_user, get_all_users::get_all_users, get_user::get_user},
    types::{
        api::{AllUsers, UserCreated},
        engine::EngineMessage,
    },
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
                EngineMessage::GetAllUsers { reply } => {
                    let users = engine.get_all_users();
                    reply.send(AllUsers { users }).unwrap()
                }
                EngineMessage::GetUser { user_id, reply } => {
                    let user = engine.get_user(user_id).unwrap();
                    reply.send(user).unwrap()
                }
                EngineMessage::CreateMarket {
                    title,
                    resolution_time,
                    reply,
                } => {
                    let market = engine.create_market(resolution_time, title);
                    reply.send(market).unwrap()
                }
                EngineMessage::DeleteMarket { market_id, reply } => {
                    let res = engine.delete_market(market_id).unwrap();
                    reply.send(res).unwrap();
                }
                EngineMessage::GetAllMarkets { reply } => {
                    let markets = engine.get_all_markets();
                    reply.send(markets).unwrap()
                }
                EngineMessage::ResolveMarket {
                    market_id,
                    reply,
                    outcome,
                } => {
                    engine.resolve_market(market_id, outcome)?;
                    reply.send(());
                }
            }
        }
    });
    let shared_state = Arc::new(Mutex::new(AppState { tx }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .route("/", get(create_user))
        .route("/users", get(get_all_users))
        .route("/user", post(get_user))
        .with_state(shared_state);

    axum::serve(listener, router).await.unwrap()
}
