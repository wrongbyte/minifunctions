use axum::{
    routing::{get, post},
    Router,
};
use deno_core::{error::AnyError, op2, Extension, JsRuntime, OpDecl, RuntimeOptions};
use routes::{deploy_new_function, execute_tenant_funcion};
mod errors;
mod routes;

#[derive(Clone)]
pub struct AppState {}

#[op2(fast)]
pub fn op_hello() {
    println!("Hello from Rust");
}

// TODO: remove tenant from path
#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let app_state = AppState {};
    let app: Router<AppState> = Router::new()
        .route("/{tenant}/deploy", post(deploy_new_function))
        .route(
            "/{tenant}/functions/{function_id}",
            get(execute_tenant_funcion),
        )
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
