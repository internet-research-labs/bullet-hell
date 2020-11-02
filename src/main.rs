mod zone;
mod hub;

use tokio::sync::mpsc;
use tokio::sync::RwLock;

use futures::{FutureExt, StreamExt};
use std::sync::Arc;

use warp::Filter;
use warp::ws::Message;


type SocketSender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

/// ...
/// ...
async fn connect(socket: warp::ws::WebSocket, h: ArcHub) {

    let _conn = h.write().await.new_conn();

    let (user_ws_tx, _user_ws_rx) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    loop {
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        send(tx.clone()).await;
    }

    println!("CLOSED!")
}

async fn send(tx: SocketSender) {
    let _ = tx.send(Ok(Message::text("!!!")));
}

type ArcHub = Arc<RwLock<hub::Hub>>;

#[tokio::main]
async fn main() {

    let hub = ArcHub::default();
    let hub = warp::any().map(move || hub.clone());

    let index = warp::path::end()
        .and(warp::fs::file("www/static/index.html"));

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("www/static/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(hub)
        .map(|ws: warp::ws::Ws, h: ArcHub| {
            ws.on_upgrade(move |websocket| connect(websocket, h))
        });

    let statics = warp::path("static")
        .and(warp::fs::dir("www/static"));

    // Compose all routes
    let routes = index.or(favicon).or(websocket).or(statics);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
