pub mod ws_server;

use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::net::SocketAddr;

pub async fn start_ws_server(addr: SocketAddr) {
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind address");
    loop {
        if let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    while let Some(message) = ws_receiver.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    ws_sender.send(Message::text("Hello from server!")).await.expect("Failed to send message");
                }
            }
            Err(_) => {
                // Handle the error
            }
        }
    }
}
