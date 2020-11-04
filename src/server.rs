mod fake;
mod zone;
mod gol;

use std::sync::Arc;
// use std::sync::mpsc;
use std::sync::atomic::{AtomicI64, Ordering};

use tokio::sync::mpsc as tmpsc;
use tokio::sync::RwLock as TokioRwLock;

use futures::{FutureExt, StreamExt, join};

use warp::Filter;
use gol::World as _WorldTrait;


const DUR: std::time::Duration = std::time::Duration::from_millis(200);

static UUID: AtomicI64 = AtomicI64::new(1);


/// Handle user connected:
/// 1. Register user with Hub
/// 2. Setup rx for game updates
/// 3. Setup tx for user inputs
/// ...
async fn connect(socket: warp::ws::WebSocket, to_game: tmpsc::UnboundedSender<String>, users: fake::Users) {

    let uuid = UUID.fetch_add(1, Ordering::Relaxed);

    let (user_ws_tx, mut user_ws_rx) = socket.split();

    let (update_tx, update_rx) = tmpsc::unbounded_channel();

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

/*
 * XXX: Remove when not needed
fn utc_now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64
}
*/

/// Return an update channel for the world in a different process!
fn world_loop() -> (tmpsc::UnboundedSender<String>, fake::Users) {

    let users = fake::Users::default();


    // Share between spawned processes
    let w = gol::GameOfLife::with_size(100, 100);
    let w = Arc::new(TokioRwLock::new(w));
    let (tx, mut rx): (tmpsc::UnboundedSender<String>, tmpsc::UnboundedReceiver<String>) = tmpsc::unbounded_channel();

    // Tick loop
    let w_tick = w.clone();
    tokio::spawn(async move {
        loop {
            let timer = std::time::Instant::now();

            let up = {
                let mut w = w_tick.write().await;
                w.tick();
                let r = w.to_string();
                r
            };
            tx.send(up).unwrap();

            let d = timer.elapsed();

            if DUR > d {
                tokio::time::delay_for(DUR-d).await
            }
        }
    });

    // Send to users loop
    let u = users.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            for (_, tx) in u.write().await.iter() {
                let m = msg.clone();
                if let Err(e) = tx.send(Ok(warp::ws::Message::text(m))) {
                    println!("ERROR: {}", e);
                }
            }
        }
    });

    // Read inputs from users
    let (tx, mut rx): (tmpsc::UnboundedSender<String>, tmpsc::UnboundedReceiver<String>) = tmpsc::unbounded_channel();
    let w_update = w.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            let w = w_update.write().await;
            let m = msg.clone();
            w.update(m);
        }
    });

    (
        tx,
        users.clone(),
    )
}


#[tokio::main]
async fn main() {

    // Shared between the read and write queue
    // let w = gol::GameOfLife::with_size(100, 100);

    // XXX: Later use this to send updates to players + receiver updates from players
    let (tx, users) = world_loop();
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
            ws.on_upgrade(move |websocket| connect(websocket, c, u))
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
