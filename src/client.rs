use futures_util::{sink::SinkExt, stream::StreamExt};

use tokio_tungstenite::{
    connect_async,
    tungstenite::{protocol::Message},
};

const N_CLIENTS: usize = 10;
const N_MESSAGES: usize = 10;
const SERVER_ADDR: &str = "ws://0.0.0.0:3000/ws";

#[tokio::main]
async fn main() {

    let mut clients = Vec::new();

    for id in 0..N_CLIENTS {
        clients.push(tokio::spawn(create_client(id)));
    }

    for client in clients {
        if let Err(error) = client.await {
            eprintln!("Client task failed: {error}");
        }
    }

    println!("All clients finished/disconnected.");

}

async fn create_client(id: usize) {
    let (ws_stream, _) = match connect_async(SERVER_ADDR).await {
        Ok(connection) => connection,
        Err(error) => {
            eprintln!("Client {id} failed due to: {error}");
            return;
        }
    };

    println!("Client {id} connected to the server");

    let (mut sender, mut receiver) = ws_stream.split();

    let mut send_task = tokio::spawn(async move {
        for i in 0..N_MESSAGES {
            if sender 
                .send(Message::Text(format!("Message {i}..").into()))
                .await
                .is_err()
            {
                eprintln!("Client {id} failed to send message {i}");
                return;
            }
        }
    });

    println!("Client {id} finished sending messages");

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Msg)) = receiver.next().await {
            process_message(id, Msg).await;
        }
    });
}

async fn process_message(id: usize, msg: Message) {
    match msg {
        Message::Text(text) => {
            println!("Client {id} received: {text}");
        }
        Message::Binary(data) => {
            println!("Client {id} received binary data: {:?}", data);
        }
        Message::Ping(ping) => {
            println!("Client {id} received ping: {:?}", ping);
        }
        Message::Pong(pong) => {
            println!("Client {id} received pong: {:?}", pong);
        }
        Message::Close(close) => {
            println!("Client {id} received close: {:?}", close);
        }
        Message::Frame(_) => {
            println!("Something entirely wrong happened with the client {id}");
        }
    }
}





