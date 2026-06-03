use std::sync::Arc;

use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, oneshot};
use uuid::Uuid;

use crate::{
    AppState,
    api::types::{api::UserCreated, engine::EngineMessage},
    error::CustomError,
    types::engine::User,
};
#[derive(Serialize, Deserialize)]
pub struct GetUserData {
    user_id: Uuid,
}

pub async fn get_user(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<GetUserData>,
) -> Result<Json<User>, CustomError> {
    let (s, r) = oneshot::channel::<User>();

    let a = state
        .lock()
        .await
        .tx
        .send(EngineMessage::GetUser {
            user_id: payload.user_id,
            reply: s,
        })
        .await
        .map_err(|_| CustomError::EngineDown);

    let x = r.await.map_err(|_| CustomError::EngineDown)?;
    Ok(Json(x))
}
