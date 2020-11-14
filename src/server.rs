mod gol;
mod since;
mod zone;

mod conceptual;

use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use futures::{SinkExt, StreamExt, join};

use clap::{App, Arg};
use tokio::sync::watch;
use tokio::sync::mpsc as tmpsc;
use tokio::sync::RwLock as TokioRwLock;
use warp::Filter;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::GzEncoder;

const DUR: std::time::Duration = std::time::Duration::from_millis(33);


static UUID: AtomicI64 = AtomicI64::new(1);


use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type Users = Arc<RwLock<HashMap<i64, UpdateSender>>>;


#[derive(Clone)]
struct PlayerReq {
    pub id: i64,
    pub msg: String,
}


/// Handle user connected:
/// 1. Register user with Hub
/// 2. Setup rx for game updates
/// 3. Setup tx for user inputs
/// ...
async fn connect(socket: warp::ws::WebSocket, to_game: tmpsc::UnboundedSender<PlayerReq>, users: Users) {

    let uuid = UUID.fetch_add(1, Ordering::Relaxed);

    let (mut user_ws_tx, mut user_ws_rx) = socket.split();

    let (update_tx, mut update_rx) = tmpsc::unbounded_channel();

    println!("Connected: {}", uuid);
    users.write().await.insert(uuid, update_tx.clone());

    // Game -> User

    // let mut tick = since::Timer::now();

    tokio::spawn(async move {
        while let Some(result) = update_rx.next().await {

            let up = result.unwrap();
            
            if let Err(e) = user_ws_tx.send(up).await {
                println!("ERROR: {}", e);
            }
        }
    });

    // User -> Game
    while let Some(result) = user_ws_rx.next().await {

        let s = match result {
            Err(e) => {
                eprintln!("{}", e);
                break;
            },
            Ok(m) => {
                m
            },
        };

        let s = match s.to_str() {
            Err(_) => {
                continue;
            },
            Ok(s) => {
                s
            },
        };

        let req = PlayerReq {
            id: uuid,
            msg: s.to_string(),
        };

        if let Err(e) = to_game.send(req) {
            println!("Error: {}", e);
        }
    }

    println!("Disconnected: {}", uuid);
    users.write().await.remove(&uuid);
}

/// Return an update channel for the world in a different process!
fn world_loop(w: (impl conceptual::World + Send + Sync + 'static)) -> (tmpsc::UnboundedSender<PlayerReq>, Users) {

    let users = Users::default();

    // Share between spawned processes
    let (world_tx, mut world_rx) = watch::channel(w.to_string());

    let w = Arc::new(TokioRwLock::new(w));

    // Tick loop
    let w_tick = w.clone();
    tokio::spawn(async move {
        let mut i = 0;

        // This loop runs at [50, 55) millis (which is very close to DUR)
        loop {
            let timer = std::time::Instant::now();

            // let up = {
            {
                let mut w = w_tick.write().await;
                // ~2min
                i = (i+1) % (60);

                if i == 0 {
                    w.reset();
                }
                w.tick();

                if let Err(_) = world_tx.broadcast(w.to_string()) {
                    // Failed
                }
            }

            // tx.send(up).unwrap();

            let d = timer.elapsed();

            if DUR > d {
                tokio::time::delay_for(DUR-d).await
            }
        }
    });

    // Read inputs from users
    let (tx, mut rx): (tmpsc::UnboundedSender<PlayerReq>, tmpsc::UnboundedReceiver<PlayerReq>) = tmpsc::unbounded_channel();
    let w_update = w.clone();
    
    // Receive updates from game
    let u = users.clone();
    tokio::spawn(async move {
        while let Some(req) = rx.next().await {
            match req.msg.as_str() {
                "*" => {
                    if let Some(up) = world_rx.recv().await {
                        {
                            let users = u.write().await;
                            
                            if let Some(u) = users.get(&req.id) {

                                let mut e = GzEncoder::new(Vec::new(), Compression::default());

                                if let Err(_) = e.write_all(up.as_bytes()) {
                                    continue;
                                };

                                let compressed = match e.finish() {
                                    Ok(r) => {
                                        r
                                    },
                                    Err(_) => {
                                        continue
                                    },
                                };

                                // println!("{} vs. {}", up.len(), compressed.len());

                                if let Err(_) = u.send(Ok(warp::ws::Message::binary(compressed))) {
                                    // Failed
                                }
                            }
                        }
                    }
                },
                _ => {
                    let w = w_update.write().await;
                    let m = req.msg.clone();
                    w.update(m);
                },
            }
        }
    });

    (
        tx,
        users.clone(),
    )
}


#[tokio::main]
async fn main() {


    let matches = App::new("bullet-hell")
        .version("0.3.1")
        .arg(Arg::with_name("port")
            .short("p")
            .value_name("PORT"))
        .arg(Arg::with_name("path")
            .long("path")
            .value_name("STATIC_PATH"))
        .arg(Arg::with_name("width")
            .short("w")
            .value_name("WIDTH"))
        .arg(Arg::with_name("height")
            .short("h")
            .value_name("HEIGHT"))
        .get_matches();


    let w = matches.value_of("width").unwrap_or("100").parse::<usize>().unwrap();
    let h = matches.value_of("height").unwrap_or("100").parse::<usize>().unwrap();
    let p = matches.value_of("port").unwrap_or("9004").parse::<u16>().unwrap();
    let path = matches.value_of("path").unwrap_or("www/static").to_string();

    println!("BULLET-HELL!");
    println!("============");
    println!("width .......... {}", w);
    println!("height ......... {}", h);
    println!("port ........... {}", p);
    println!("static-path .... {}", path);
    println!("");

    // Shared between the read and write queue
    // let w = gol::GameOfLife::with_size(100, 100);

    // XXX: Later use this to send updates to players + receiver updates from players
    let w = gol::GameOfLife::with_size(h, w);
    let (tx, users) = world_loop(w);
    let usr = warp::any().map(move || users.clone());
    let com = warp::any().map(move || tx.clone());

    let q = path.clone();
    println!("{}", q);
    let index = warp::path::end()
        .and(warp::fs::file(q + "/index.html"));

    let q = path.clone();
    let favicon = warp::path("favicon.ico")
        .and(warp::fs::file(q + "/favicon.ico"));

    let websocket = warp::path("ws")
        .and(warp::ws())
        .and(com)
        .and(usr)
        .map(|ws: warp::ws::Ws, c, u| {
            ws.on_upgrade(move |websocket| connect(websocket, c, u))
        });

    let q = path.clone();
    let statics = warp::path("static")
        .and(warp::fs::dir(q));

    // Compose all routes
    let routes = index.or(favicon)
        .or(websocket)
        .or(statics);

    let server = warp::serve(routes)
        .run(([0, 0, 0, 0], p));

    join!(
        server,
    );
}
