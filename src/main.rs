use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::prelude::*;

fn initialize_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("debug"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() {
    initialize_tracing();

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/rust-analyzer-lsp-websocket",
            get(rust_analyzer_lsp_websocket_handler),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn rust_analyzer_lsp_websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_rust_analyzer_lsp_websocket)
}

async fn handle_rust_analyzer_lsp_websocket(mut socket: WebSocket) {}
