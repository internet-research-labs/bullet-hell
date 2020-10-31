mod zone;
mod hub;

// use tokio::sync::{mpsc, RwLock};
use tokio::sync::RwLock;

use std::sync::Arc;

use warp::Filter;

async fn connect(_ws: warp::ws::WebSocket, hub: ArcHub) {
    let id = hub.write().await.uuid();
    println!("Connected... {}", id)
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
        .map(|ws: warp::ws::Ws, h| {
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
