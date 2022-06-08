use std::net::SocketAddr;

use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::env;
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::prelude::*;
use url::Url;

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

#[derive(Clone)]
struct Args {
    cwd: Url,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    initialize_tracing();

    let args = Args {
        cwd: Url::from_directory_path(&env::current_dir()?).expect("valid url from current dir"),
    };

    let app = Router::new()
        .layer(Extension(args))
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/rust-analyzer-lsp-websocket",
            get(rust_analyzer_lsp_websocket_handler),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    debug!(?addr, "listening");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn rust_analyzer_lsp_websocket_handler(
    ws: WebSocketUpgrade,
    Extension(args): Extension<Args>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_rust_analyzer_lsp_websocket(socket, args))
}

async fn handle_rust_analyzer_lsp_websocket(mut socket: WebSocket, args: Args) {}
