mod fake;
mod zone;
mod gol;

use tokio::sync::mpsc;
use tokio::sync::RwLock;

use futures::{FutureExt, StreamExt, join};

use warp::Filter;
use gol::World as _WorldTrait;


/// UPDATES
async fn updates(walkie: fake::Walkie, users: fake::Users) {

    const DUR: std::time::Duration = std::time::Duration::from_millis(1000);

    // Shared between the read and write queue
    let w = gol::GameOfLife::with_size(100, 100);
    let w = RwLock::new(w);

    // Send to async loop
    let _tx = walkie.tx.clone();

    let ticker = async {
        loop {
            {
                w.write().await.tick();
            }
            tokio::time::delay_for(DUR).await;
        }
    };

    let fin = async {
        loop {

            let update_msg = {
                w.read().await.to_string()
            };

            for (_, tx) in users.write().await.iter() {
                let m = update_msg.clone();
                if let Err(e) = tx.send(Ok(warp::ws::Message::text(m))) {
                    println!("ERROR: {}", e);
                }
            }

            tokio::time::delay_for(DUR).await;
        }
    };

    // Gets burrowed by while loop
    let mut rx = walkie.rx;

    let modifier = async {
        while let Some(msg) = rx.next().await {
            let w = w.write().await;
            w.update(msg);
        }
    };

    join!(
        fin,
        modifier,
        ticker,
    );
}

use std::sync::atomic::{AtomicI64, Ordering};

static UUID: AtomicI64 = AtomicI64::new(1);

/// Handle user connected:
/// 1. Register user with Hub
/// 2. Setup rx for game updates
/// 3. Setup tx for user inputs
/// ...
async fn fake_connect(socket: warp::ws::WebSocket, to_game: mpsc::UnboundedSender<String>, users: fake::Users) {

    let uuid = UUID.fetch_add(1, Ordering::Relaxed);

    let (user_ws_tx, mut user_ws_rx) = socket.split();

    let (update_tx, update_rx) = mpsc::unbounded_channel();

    users.write().await.insert(uuid, update_tx.clone());

    tokio::task::spawn(update_rx.forward(user_ws_tx).map(move |result| {
        println!("...");
        if let Err(e) = result {
            eprintln!("[{}] websocket send error: {}", "-", e);
        }
    }));

    // Player -> World
    while let Some(result) = user_ws_rx.next().await {
        let _ = match result {
            Ok(msg) => {
                let _msg = if let Ok(s) = msg.to_str() {
                    if let Err(e) = to_game.send(s.to_string()) {
                        println!("Error: {}", e);
                    }
                } else {
                    println!("!!!!");
                    break;
                };
            },
            Err(_) => {
                println!("????");
                break;
            },
        };
    }

    users.write().await.remove(&uuid);
}

#[tokio::main]
async fn main() {

    let (walkie, tx, _rx) = fake::Walkie::gen();

    let users = fake::Users::default();
    let u = users.clone();

    tokio::task::spawn(async move {
        updates(walkie, u).await;
    });

    let usr = warp::any().map(move || users.clone());
    let com = warp::any().map(move || tx.clone());

    let index = warp::path::end()
        .and(warp::fs::file("www/static/index.html"));

    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file("www/static/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(com)
        .and(usr)
        .map(|ws: warp::ws::Ws, c, u| {
            ws.on_upgrade(move |websocket| fake_connect(websocket, c, u))
        });

    let statics = warp::path("static")
        .and(warp::fs::dir("www/static"));

    // Compose all routes
    let routes = index.or(favicon)
        .or(websocket)
        .or(statics);

    let server = warp::serve(routes)
        .run(([127, 0, 0, 1], 8080));

    join!(
        server,
    );
}
