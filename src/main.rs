use std::sync::{Arc, LazyLock};

use anyhow::Context;
use axum::Router;
use deno_core::{error::AnyError, op2, Extension, JsRuntime, OpDecl, RuntimeOptions};
use sqlx::{postgres::PgPoolOptions, Pool};
use tenants::repository::{DynTenantRepository, SqlTenantRepository};
use uuid::Uuid;
mod errors;
mod functions;
mod tenants;

pub static TENANT_ID_TEST: LazyLock<Uuid> = LazyLock::new(|| Uuid::now_v7());

#[derive(Clone)]
pub struct AppState {
    pub tenants_repository: Arc<DynTenantRepository>,
}

#[op2(fast)]
pub fn op_hello() {
    println!("Hello from Rust");
}

// TODO: remove tenant from path
#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let host = std::env::var("DATABASE_URL").expect("failed to get database url");
    dbg!(&host);
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&std::env::var("DATABASE_URL").expect("failed to get database url"))
        .await
        .context("failed to connect to database")?;
    let tenants_repository = SqlTenantRepository { pool };

    let app_state = AppState {
        tenants_repository: Arc::new(tenants_repository),
    };

    let app: Router<AppState> = Router::new()
        .nest("/functions", functions::routes::router())
        .nest("/tenants", tenants::routes::router())
        .with_state(app_state);

    const DECL: OpDecl = op_hello();
    let ext = Extension {
        name: "ops",
        ops: std::borrow::Cow::Borrowed(&[DECL]),
        ..Default::default()
    };

    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    let result = runtime
        .execute_script("", include_str!("js/hello.js"))
        .unwrap();
    dbg!(result);
    Ok(())
}
