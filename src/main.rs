// mod zone;
// use zone::Pos;
// use zone::Coordinate;

mod zone;
mod hub;

// use tokio::sync::{mpsc, RwLock};
use warp::Filter;

async fn connect(_ws: warp::ws::WebSocket, id: i64) {
    println!("Connected... id = {}", id);
}

#[tokio::main]
async fn main() {

    // NOTE: This seems pretty sketch...
    // Hub shouldn't be cloned for every connection (???)
    let zzz = hub::Hub::new();
    let zzz = warp::any().map(move || zzz.uuid());

    let index = warp::path::end()
        .and(warp::fs::file("www/static/index.html"));

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("www/static/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(zzz)
        .map(|ws: warp::ws::Ws, uuid| {
            ws.on_upgrade(move |socket| connect(socket, uuid))
        });

    let statics = warp::path("static")
        .and(warp::fs::dir("www/static"));

    // Compose all routes
    let routes = index.or(favicon).or(websocket).or(statics);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
