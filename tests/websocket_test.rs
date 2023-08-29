use tokio::net::TcpStream;
use tungstenite::client;
use tokio_tungstenite::tungstenite;
use tokio::sync::oneshot;

#[tokio::test]
async fn ws_test() {
    let (ready_tx, ready_rx) = oneshot::channel();
    let server_address = "127.0.0.1:12345".parse().unwrap();
    
    // Start WebSocket server
    let ws_server_handle = tokio::spawn(async {
        crate::ws_server::start_ws_server(server_address, ready_tx).await.unwrap();
    });

    // Wait for server to be ready
    ready_rx.await.unwrap();

    // Create a client connection.
    let (socket, _) = client::connect_async(format!("ws://{}", server_address)).await.expect("Failed to connect");
    let (mut write, mut read) = socket.split();

    // Send message from client to server
    write.send(tungstenite::protocol::Message::Text("Hello from client!".into())).await.unwrap();

    // Receive server response
    let msg = read.next().await.unwrap().unwrap();
    let content = msg.to_text().unwrap();
    assert_eq!(content, "Hello from client!");

    // Close server
    ws_server_handle.abort();
}
