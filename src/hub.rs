/**
 * (>x_x)> W         E R
 * (>x_x)>  H       V
 * (>x_x)>    A   E (>x_x)>      T 
 */

use std::sync::mpsc;
use std::collections::HashMap;
use tokio::sync::RwLock;


use std::sync::Arc;

pub type UpdateReceiver = mpsc::Receiver<i64>;
pub type UpdateSender = mpsc::Sender<i64>;


type ConnectionMap = Arc<RwLock<HashMap<i64, UpdateReceiver>>>;


/// A connection is represented here
#[derive(Clone)]
pub struct HubConn {
    pub id: i64,
    inputs: UpdateSender,
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
    // conns: ConnectionMap,
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
            // XXX: Uncomment
            // conns: ConnectionMap::default(),
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

    /// Returns a two-way connection to this Hub.
    /// ```
    /// use hub::Hub;
    /// assert!(false);
    /// ```
    pub fn new_conn(&mut self,  client_receiver: UpdateReceiver) -> HubConn {
        let (sender, receiver): (UpdateSender, UpdateReceiver) = mpsc::channel();

        let id = self.uuid();

        HubConn {
            id: id,
            inputs: sender,
        }
    }
}
