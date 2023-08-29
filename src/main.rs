#[macro_use]
extern crate lazy_static;

use std::net::SocketAddr;
use std::env;
use dotenv::dotenv;
use rabbi_rust::ws_server;
use tokio::sync::oneshot;
use std::error::Error;
use tokio::signal::ctrl_c;
use chrono::Utc;
use serde_json::json;

mod log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Read WebSocket address from environment variable
    let ws_addr_str = env::var("WS_SERVER_ADDR").expect("WS_SERVER_ADDR must be set");
    let ws_addr: SocketAddr = ws_addr_str.parse().expect("Invalid address format");

    // Create channel to signal that the WebSocket server is ready
    let (ready_tx, ready_rx) = oneshot::channel();

    // Start WebSocket Server
    let ws_server_handle = tokio::spawn(async move {
        if let Err(e) = ws_server::start_ws_server(ws_addr, ready_tx).await {
            eprintln!("WebSocket Server error: {}", e);
        }
    });

    // Wait for the WebSocket server to be ready
    ready_rx.await.expect("Failed to get ready signal from WebSocket server");

    slog::info!(*log::LOGGER, "WebSocket server is ready."; "timestamp" => format!("{:?}", std::time::SystemTime::now()));

    // Your existing or additional code to run the HTTP server or other tasks


    ctrl_c().await.unwrap();

    ws_server_handle.await.unwrap();

    println!("Server has shut down");

    Ok(())
}
