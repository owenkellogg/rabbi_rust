use futures_util::{StreamExt, SinkExt};
use warp::Filter;
use std::net::SocketAddr;
use tokio::sync::oneshot;

pub async fn start_ws_server(addr: SocketAddr, ready_tx: oneshot::Sender<()>) -> Result<(), warp::Error> {
    let ws_route = warp::path("socket")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket: warp::ws::WebSocket| {
                let (mut tx, mut rx) = websocket.split();
                async move {
                    while let Some(result) = rx.next().await {
                        let msg = match result {
                            Ok(msg) => msg,
                            Err(_) => {
                                break;
                            }
                        };
                        let _ = tx.send(msg).await;
                    }
                }
            })
        });

    let (_, server) = warp::serve(ws_route).bind_with_graceful_shutdown(addr, async {
        ready_tx.send(()).unwrap();
    });

    server.await;
    Ok(())
}
