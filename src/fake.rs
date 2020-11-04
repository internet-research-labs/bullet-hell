use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type Users = Arc<RwLock<HashMap<i64, UpdateSender>>>;
