/**
 * (>x_x)> W         E R
 * (>x_x)>  H       V
 * (>x_x)>    A   E (>x_x)>      T 
 */


use std::collections::HashMap;

use warp;
use tokio::sync::mpsc;


pub type UpdateReceiver = mpsc::UnboundedReceiver<Result<warp::ws::Message, warp::Error>>;
pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type ConnectionMap = HashMap<i64, UpdateSender>;

pub type ArcHub = Arc<RwLock<Hub>>;


/// A connection is represented here
#[derive(Clone)]
pub struct HubConn {
    pub id: i64,
    pub tx: UpdateSender,
}

impl HubConn {
}

impl std::fmt::Display for HubConn {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!(
            "HubConn[id={}, {}]",
            self.id,
            "...",
        ))
    }
}

/// A Hub is represented here. Hubs receive communication from the player, and send game updates
/// from the game zones.
pub struct Hub {
    curr: i64,

    // Outbound connections to clients (publish game state)
    conns: ConnectionMap,

    // User input inbound receiver (listen to for client updates)
    pub rx: UpdateReceiver,

    // User input inbound transmitter (give to connections)
    pub tx: UpdateSender,
}

impl Hub {

    /// Returns a new communication "Hub".
    /// ```
    /// use hub::Hub;
    /// let h = Hub::new();
    /// ```
    pub fn new() -> Hub {
        let (tx, rx): (UpdateSender, UpdateReceiver) = mpsc::unbounded_channel();
        Hub {
            // curr: AtomicUsize::new(1),
            curr: 1,
            conns: ConnectionMap::default(),
            rx: rx,
            tx: tx,
        }
    }

    /// Returns a unique identifier. This will be deprecated (and encapsulated) within `new_conn`.
    /// ```
    /// use hub::Hub;
    /// let h = Hub::new();
    /// assert!(h.uuid(), 1);
    /// assert!(h.uuid(), 2);
    /// assert!(h.uuid(), 3);
    /// ```
    pub fn uuid(&mut self) -> i64 {
        self.curr += 1;
        self.curr
    }

    /// Removes a connection via uuid.
    /// ```
    /// use hub::Hub;
    /// h.remove(&10);
    /// ```
    pub fn remove(&mut self, id: i64) {
        self.conns.remove(&id);
    }

    /// Returns a two-way connection to this Hub.
    /// ```
    /// use hub::Hub;
    /// assert!(false);
    /// ```
    pub fn new_conn(&mut self, tx: UpdateSender) -> HubConn {
        let id = self.uuid();

        self.conns.insert(id, tx.clone());

        // NOTE: We are receiving updates on rx, but not doing anything with it...
        // XXX: Start here
        // let (tx, _rx) = mpsc::unbounded_channel();

        HubConn {
            id: id,
            tx: self.tx.clone(),
        }
    }

    /// Send a message to every connected user.
    /// NOTE: In go you'd just have a channel working as fast as possible, and this probably will
    /// have an issue later in the project, but is fine for now.
    pub fn broadcast(&self, message: String) {
        for (&id, tx) in self.conns.iter() {
            if let Err(e) = tx.send(Ok(warp::ws::Message::text(message.clone()))) {
                println!("ERROR[{}]: {}", id, e);
            }
        }
    }
}



use std::sync::Arc;
use tokio::sync::RwLock;

pub type WhatReceiver = mpsc::UnboundedReceiver<String>;
pub type WhatSender = mpsc::UnboundedSender<String>;
pub type WhatConns = HashMap<i64, WhatSender>;

#[derive(Clone)]
pub struct WhatHub {
    curr: i64,
    pub conns: Arc<RwLock<WhatConns>>,
    pub rx: Arc<RwLock<mpsc::UnboundedReceiver<String>>>,
}

impl WhatHub {
    pub fn new () -> WhatHub {
        let (tx, rx) = mpsc::unbounded_channel();
        WhatHub {
            curr: 1,
            conns: Arc::new(RwLock::new(WhatConns::new())),
            rx: Arc::new(RwLock::new(rx)),
        }
    }

    pub fn receive_updates(&self, tx: mpsc::UnboundedSender<String>) {

    }
}

pub type ArcWhatHub = Arc<RwLock<WhatHub>>;


pub fn Conns() -> (mpsc::UnboundedSender<String>, i64, ArcWhatHub) {

    // Update inbound to the game
    // let (gamebound_tx, gamebound_rx) = mpsc::unbounded_channel();

    // Update outboundto the users
    let (playerbound_tx, playerbound_rx) = mpsc::unbounded_channel();

    // ...
    let hub = Arc::new(RwLock::new(WhatHub{
        curr: 1,
        conns: Arc::new(RwLock::new(WhatConns::default())),
        rx: Arc::new(RwLock::new(playerbound_rx)),
    }));

    // ...
    return (
        playerbound_tx,
        -1,
        hub,
    )
}
