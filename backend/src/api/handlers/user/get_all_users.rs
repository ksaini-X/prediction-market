use std::sync::Arc;

use axum::{Json, extract::State};
use tokio::sync::{Mutex, oneshot};

use crate::{
    AppState,
    api::types::{api::AllUsers, engine::EngineMessage},
    error::CustomError,
};
pub async fn get_all_users<'a>(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<AllUsers>, CustomError> {
    let (s, r) = oneshot::channel::<AllUsers>();
    let a = state
        .lock()
        .await
        .tx
        .send(EngineMessage::GetAllUsers { reply: s })
        .await
        .map_err(|_| CustomError::EngineDown);

    let x = r.await.map_err(|_| CustomError::EngineDown)?;
    Ok(Json(x))
}
