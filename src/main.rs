use axum:: {
    body:: {to_bytes, Body},
    extract::ws:: {WebSocketUpgrade, WebSocket },
    routing::any,
    response:: { IntoResponse},
    Router,
};
use axum_extra::TypedHeader;

//use std::ops::ControlFlow;
use std::{net::SocketAddr};

//use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::extract::connect_info::ConnectInfo;
//use axum::extract::ws::CloseFrame;

//use futures_util::{sink::SinkExt, stream::StreamExt};

#[warn(unused_imports)]

#[tokio::main]
async fn main() {

    let app = Router::new().route("/ws", any(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .await;
}

async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("unknown Browser")
    };
    println!("`{user_agent}` at {addr} connected.");

}

async fn handle_socket(who: SocketAddr) {
    println!("Handling socket for {who}");
}

