use std::sync::Arc;

use axum::{Json, extract::State};
use tokio::sync::{Mutex, oneshot};

use crate::{
    AppState,
    api::types::{api::UserCreated, engine::EngineMessage},
    error::CustomError,
};
pub async fn create_user(
    State(state): State<Arc<AppState>>,
) -> Result<Json<UserCreated>, CustomError> {
    let (s, r) = oneshot::channel::<UserCreated>();
    state
        .tx
        .send(EngineMessage::CreateUser { reply: s })
        .await
        .map_err(|_| CustomError::EngineDown)?;

    let x = r.await.map_err(|_| CustomError::EngineDown)?;
    Ok(Json(x))
}
