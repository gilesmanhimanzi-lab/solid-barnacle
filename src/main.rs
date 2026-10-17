use axum:: {
    body:: {to_bytes, Body},
    extract::ws:: {Message, Utf8Bytes, WebSocketUpgrade, WebSocket },
    routing::any,
    response:: { IntoResponse, Response},
    Router,
};
use axum_extra::TypedHeader;

use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer};
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

use features_util::{sink::SinkExt, stream::streamExt};

#[tokio::main]
async fn main() {
    tracing_subscriber.registry()
        .with(
            tracing_subscriber::Envfilter().try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower-http =debug", env!("CARGO_CRATE_NAME")).into()
            })
        )
        .with(tracing_subscriber::fmt::layer())
        .init()

    let asset_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR").join("assets"));

    //a single router used by app
    let app = Router::new()
        .fallback_service()
        .route("/ws", get(handler))
        .layer();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr.unwrap());
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    )
    .await;
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
};

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        
    }
}
