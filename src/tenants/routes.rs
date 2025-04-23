use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{errors::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(StatusCode::CREATED))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateTenantPayload {
    pub name: String,
}

#[axum::debug_handler]
#[tracing::instrument(skip(state))]
pub async fn deploy_new_function(
    State(state): State<AppState>,
    Json(function_code): Json<CreateTenantPayload>,
) -> Result<StatusCode, AppError> {
    
    // TODO
    Ok(StatusCode::CREATED)
}
