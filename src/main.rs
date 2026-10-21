use axum:: {
    extract::ws:: {WebSocketUpgrade, WebSocket, Message },
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

use futures_util::{StreamExt};

struct Procurement {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

#[warn(unused_imports)]

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/ws", any(handler))
        .route("/", any(front));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is listening....");

    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    )
    .await
    .expect("Server could not be started");
}

async fn handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = user_agent
    .map(|TypedHeader(agent)| agent.to_string())
    .unwrap_or_else(||"unknown client".to_string());
    
    println!("Client Connected: {user_agent} at {addr}.");

    ws.on_upgrade(move |socket: WebSocket| handle_socket(socket, addr))
    

}

async fn front() -> impl IntoResponse {
    "Hello, Welcome to the front"
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
    println!("Handling socket for {addr}");

    while let Some(result) = socket.next().await {
        match result {
            Ok(Message::Text(text)) => {
                println!("Message from {addr}: {text}");

                let response = format!("Server got: {text}");

                if socket
                    .send(Message::Text(response.into()))
                    .await
                    .is_err()

                {
                    println!("Failed to  send message to {addr}");
                    break;
                }
            }

            Ok(Message::Close(_)) => {
                println!("{addr} disconnected");
                break;
            }

            Ok(_) => {

            }

            Err(error) => {
                println!("Error from {addr}: {error}");
                break;
            }
        }
    }

    println!("COnnection closed for {addr}");
}

