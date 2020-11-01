mod zone;
mod hub;

// use tokio::sync::{mpsc, RwLock};
use tokio::sync::RwLock;

use std::sync::Arc;
use std::sync::mpsc;

use warp::Filter;

async fn connect(_ws: warp::ws::WebSocket, h: ArcHub) {
    let (tx, rx): (hub::UpdateSender, hub::UpdateReceiver) = mpsc::channel();
    let conn = h.write().await.new_conn(rx);


    println!("Connected... {}", conn);
}

type ArcHub = Arc<RwLock<hub::Hub>>;

#[tokio::main]
async fn main() {

    let hub = ArcHub::default();

    let index = warp::path::end()
        .and(warp::fs::file("www/static/index.html"));

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("www/static/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || hub.clone()))
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
