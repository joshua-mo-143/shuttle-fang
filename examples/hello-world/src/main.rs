use axum::{routing::get, Router};
use fang::{NoTls, AsyncQueue};

#[derive(Clone)]
struct AppState {
    pool: AsyncQueue<NoTls>,
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_fang::Postgres] pool: AsyncQueue<NoTls>) -> shuttle_axum::ShuttleAxum {
    let state = AppState { pool };
    let router = Router::new().route("/", get(hello_world)).with_state(state);

    Ok(router.into())
}
