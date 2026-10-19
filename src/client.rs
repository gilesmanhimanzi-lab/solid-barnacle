use futures_Util::{sink::SinkExt, stream::StreamExt};
use std::time::{Duration, Instant};

use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message},
};

const N_Clients: usize = 10;
const N_Messages: usize = 10;
const Server_Addr: &str = "ws://localhost:3000/ws";

async fn main() {
    let mut clients = (0..N_Clients).map(create_client).collect::Vec::new();
}

async fn create_client(id: usize) {
    let (ws_stream, _) = connect_async(Server_Addr).await.expect("Failed to connect");
    println!("Client {id} connected to the server");
}