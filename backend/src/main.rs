use axum::Router;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::{
    api::{
        handlers::{market::market_router, user::user_router},
        types::{
            api::{AllUsers, UserCreated},
            engine::EngineMessage,
        },
    },
    error::CustomError,
    types::orderbook::Order,
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
                EngineMessage::GetAllOrdersForUser { user_id, reply } => {
                    let orders = engine
                        .markets
                        .values()
                        .map(|market| market.orderbook.get_all_orders_for_user(user_id))
                        .flatten()
                        .collect();
                    reply.send(orders).unwrap()
                }
                EngineMessage::CancelOrder {
                    user_id,
                    order_id,
                    market_id,
                    reply,
                } => {
                    let res = engine
                        .markets
                        .get_mut(&market_id)
                        .expect("Market not found")
                        .orderbook
                        .cancel_order(order_id)
                        .unwrap();
                    reply.send(res).unwrap();
                }

                EngineMessage::PlaceOrder {
                    market_id,
                    user_id,
                    order_action: action,
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
                    let _ = reply.send(markets);
                }
                EngineMessage::ResolveMarket {
                    market_id,
                    reply,
                    outcome,
                } => {
                    let market = engine.resolve_market(market_id, outcome).unwrap();
                    let _ = reply.send(market);
                }
                EngineMessage::GetMarket { market_id, reply } => {
                    let market = engine.get_market(market_id).unwrap();
                    let _ = reply.send(market);
                }
            }
        }
    });
    let shared_state = Arc::new(AppState { tx });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let router = Router::new()
        .nest("/api", user_router())
        .nest("/api", market_router())
        .with_state(shared_state);

    axum::serve(listener, router).await.unwrap()
}
