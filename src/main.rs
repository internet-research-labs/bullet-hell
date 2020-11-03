mod fake;
mod zone;

use tokio::sync::mpsc;
use tokio::sync::RwLock;

use futures::{FutureExt, StreamExt, join};

use warp::Filter;


struct World {
    f: f64,
}

/// UPDATES
async fn updates(walkie: fake::Walkie, users: fake::Users) {

    // Shared between the read and write queue
    let w = RwLock::new(World{f: 0.0});

    // Send to async loop
    let _tx = walkie.tx.clone();

    let fin = async {
        loop {
            let w = w.read().await;

            for (_, tx) in users.write().await.iter() {
                tx.send(Ok(warp::ws::Message::text(w.f.to_string())));
            }

            tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
        }
    };

    // Gets burrowed by while loop
    let mut rx = walkie.rx;

    let modifier = async {
        while let Some(msg) = rx.next().await {
            let f = msg.parse::<f64>().ok().unwrap();
            let mut w = w.write().await;
            w.f += f;
            println!("RX+{}: {}", f, w.f);
        }
    };

    join!(
        fin,
        modifier,
    );
}

/// Handle user connected:
/// 1. Register user with Hub
/// 2. Setup rx for game updates
/// 3. Setup tx for user inputs
/// ...
async fn fake_connect(socket: warp::ws::WebSocket, to_game: mpsc::UnboundedSender<String>, users: fake::Users) {

    let (user_ws_tx, mut user_ws_rx) = socket.split();

    let (update_tx, update_rx) = mpsc::unbounded_channel();

    users.write().await.insert(0, update_tx.clone());

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
                    to_game.send(s.to_string());
                } else {
                    break;
                };
            },
            Err(_) => {
                break;
            },
        };
    }
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
