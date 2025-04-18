use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{errors::AppError, AppState};

#[derive(serde::Deserialize, Debug)]
pub struct DeployFunctionRequest {
    pub code: String,
}

#[axum::debug_handler]
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
pub async fn execute_tenant_funcion(
    State(state): State<AppState>,
    Path(path): Path<ExecuteTenantFunctionPath>,
    Json(function_code): Json<DeployFunctionRequest>,
) -> Result<StatusCode, AppError> {
    // TODO
    Ok(StatusCode::OK)
}
