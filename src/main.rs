use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
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
