/**
 * (>x_x)> W         E R
 * (>x_x)>  H       V
 * (>x_x)>    A   E (>x_x)>      T 
 */


use rand::Rng;
use std::collections::HashMap;

use warp;
use tokio::sync::mpsc;


// pub type UpdateReceiver = mpsc::UnboundedReceiver<Result<warp::ws::Message, warp::Error>>;
pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type ConnectionMap = HashMap<i64, UpdateSender>;


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
#[derive(Default, Clone)]
pub struct Hub {
    curr: i64,
    // XXX: Uncomment
    conns: ConnectionMap,
}

impl Hub {

    /// Returns a new communication "Hub".
    /// ```
    /// use hub::Hub;
    /// let h = Hub::new();
    /// ```
    pub fn new() -> Hub {
        Hub {
            // curr: AtomicUsize::new(1),
            curr: 1,
            conns: ConnectionMap::default(),
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
        // self.curr.fetch_add(1, Ordering::Relaxed)
        //    .try_into().unwrap()
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

        let (tx, rx) = mpsc::unbounded_channel();

        HubConn {
            id: id,
            tx: tx,
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
