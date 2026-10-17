use axum:: {
    extract::ws:: { WebSocketUpgrade, WebSocket },
    routing::any,
    response:: { IntoResponse, Response},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]

async fn main() {
    //a single router used by app
    let app = Router::new().route("/ws", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
};

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        
    }
}
