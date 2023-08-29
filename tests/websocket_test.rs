use my_rust_project::ws_server;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::connect_async;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::net::SocketAddr;

#[tokio::test]
async fn test_websocket_connection() {
    // Initialize WebSocket server using imported module
    let addr = "127.0.0.1:3012".parse::<SocketAddr>().unwrap();
    tokio::spawn(async move {
        ws_server::start_ws_server(addr).await;
    });

    // Wait a bit to ensure server has started.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // WebSocket client
    let (mut write, mut read) = connect_async("ws://127.0.0.1:3012")
        .await
        .unwrap()
        .0
        .split();

    write.send(Message::text("Hello, server!")).await.unwrap();
    if let Some(msg) = read.next().await {
        let msg = msg.unwrap().to_text().unwrap();
        assert_eq!(msg, "Hello from server!");
    }
}
