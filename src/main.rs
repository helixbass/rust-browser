use std::{net::SocketAddr, process::Stdio};

use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use std::env;
use tokio::process::Command;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info};
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

#[derive(Clone, Debug)]
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
        .route("/", get(|| async { "Hello, world!" }))
        .route(
            "/rust-analyzer-lsp-websocket",
            get(rust_analyzer_lsp_websocket_handler),
        )
        .layer(Extension(args))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
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

#[tracing::instrument(skip(socket, args), fields(cwd = args.cwd.path()))]
async fn handle_rust_analyzer_lsp_websocket(mut socket: WebSocket, args: Args) {
    info!("starting rust-analyzer");
    let mut server = Command::new("rust-analyzer")
        .args(&([] as [&str; 0]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .expect("Couldn't spawn rust-analyzer");
}
