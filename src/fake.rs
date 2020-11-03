use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub type HubArcRef = Arc<RwLock<Hub>>;

pub struct Hub {
}

impl Hub {
    pub fn new() -> Hub {
        Hub{}
    }
}


pub struct Walkie {
    pub tx: mpsc::UnboundedSender<String>,
    pub rx: mpsc::UnboundedReceiver<String>,
}

impl Walkie {
    pub fn gen() -> (Walkie, mpsc::UnboundedSender<String>, mpsc::UnboundedReceiver<String>) {
        let (world_updates_tx, mut world_updates_rx) = mpsc::unbounded_channel();
        let (player_updates_tx, mut player_updates_rx) = mpsc::unbounded_channel();

        let w = Walkie {
            rx: player_updates_rx,
            tx: world_updates_tx,
        };

        (
            w,
            player_updates_tx,
            world_updates_rx,
        )
    }
}

pub type UpdateReceiver = mpsc::UnboundedReceiver<Result<warp::ws::Message, warp::Error>>;
pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type Users = Arc<RwLock<HashMap<i64, UpdateSender>>>;
