// mod zone;
// use zone::Pos;
// use zone::Coordinate;

use futures::{FutureExt, StreamExt};
use warp::Filter;

#[tokio::main]
async fn main() {

    let index = warp::path::end()
        .and(warp::fs::file("www/static/index.html"));

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("www/static/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });

    let statics = warp::path("static")
        .and(warp::fs::dir("www/static"));

    // Compose all routes
    let routes = index.or(favicon).or(websocket).or(statics);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
