use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;

use crate::{errors::AppError, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/deploy", post(deploy_new_function))
        .route("/{function_id}", get(execute_tenant_function))
}

#[derive(serde::Deserialize, Debug)]
pub struct DeployFunctionRequest {
    pub code: String,
}

#[tracing::instrument(skip(state))]
pub async fn deploy_new_function(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(function_code): Json<DeployFunctionRequest>,
) -> Result<StatusCode, AppError> {
    // TODO
    Ok(StatusCode::CREATED)
}

#[derive(serde::Deserialize, Debug)]
pub struct ExecuteTenantFunctionPath {
    pub tenant_id: Uuid,
    pub function_id: Uuid,
}

#[axum::debug_handler]
#[tracing::instrument(skip(state))]
pub async fn execute_tenant_function(
    State(state): State<AppState>,
    Path(path): Path<ExecuteTenantFunctionPath>,
    Json(function_code): Json<DeployFunctionRequest>,
) -> Result<StatusCode, AppError> {
    // TODO
    Ok(StatusCode::OK)
}
