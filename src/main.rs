mod zone;
mod hub;

use tokio::sync::mpsc;
use tokio::sync::RwLock;

use futures::{FutureExt, StreamExt, join};
use std::sync::Arc;

use warp::Filter;
use warp::ws::Message;


type SocketSender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

/// Handle user connected:
/// 1. Register user with Hub
/// 2. Setup rx for game updates
/// 3. Setup tx for user inputs
/// ...
async fn connect(socket: warp::ws::WebSocket, h: ArcHub) {

    let (user_ws_tx, mut user_ws_rx) = socket.split();

    let (update_tx, update_rx) = mpsc::unbounded_channel();

    let conn = h.write().await.new_conn(update_tx.clone());

    let id = conn.id;

    // Send game updates to the user
    tokio::task::spawn(update_rx.forward(user_ws_tx).map(move |result| {
        if let Err(e) = result {
            eprintln!("[{}] websocket send error: {}", id, e);
        }
    }));

    // Receive game updates from the user
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => {
                let msg = if let Ok(s) = msg.to_str() {
                    // NOTE: This is the important part of this loop
                    s
                } else {
                    break;
                };
            }
            Err(e) => {
                break;
            },
        };
    }

    println!("Clossing connection: {}", id)
}

type ArcHub = Arc<RwLock<hub::Hub>>;

async fn game_updates(h: ArcHub) {
    loop {
        h.read().await.broadcast("Something");
        tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {

    let hub = ArcHub::default();
    let fin = game_updates(hub.clone());

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

    let server = warp::serve(routes)
        .run(([127, 0, 0, 1], 8080));

    join!(server, fin);
}
