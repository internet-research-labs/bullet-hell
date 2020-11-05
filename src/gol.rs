use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use std::vec::Vec;

use serde::{Serialize, Deserialize};
use serde_json::json;

use tokio::sync::RwLock;
use tokio::sync::mpsc;

pub type UpdateSender = mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>;
pub type Users = Arc<RwLock<HashMap<i64, UpdateSender>>>;

#[derive(Serialize, Deserialize)]
pub struct GameOfLife {
    grid: Vec<bool>,
    dims: (usize, usize),
}

impl GameOfLife {

    /// Return a random GOL of specified dimensions.
    pub fn with_size(h: usize, w: usize) -> Self {

        let mut g = Vec::with_capacity(h*w);
        g.resize_with(h*w, Default::default);

        let mut game = GameOfLife{
            grid: g,
            dims: (h, w),
        };

        game.randomize();

        game
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let (h, w) = self.dims;
        for i in 0..h {
            for j in 0..w {
                let r: f64 = rng.gen_range(0.0, 1.0);
                if r < 0.5 {
                    let p = self.pos(i, j);
                    self.grid[p] = true;
                }
            }
        }
    }

    /// Return the position of a particular cell.
    fn get(&self, i: usize, j: usize) -> bool {
        if i >= self.dims.0 {
            return false;
        } else if j >= self.dims.1 {
            return false;
        }
        return self.grid[self.pos(i, j)];
    }

    /// Return the count of neighbors for a particular cell.
    fn count(&self, i: usize, j: usize) -> usize {
        let mut count = 0;
        for di in 0..3 {
            for dj in 0..3 {
                if di == 1 && dj == 1 {
                    continue;
                }
                if i+di < 1 || j+dj < 1 {
                    continue;
                }
                count += self.get(i+di-1, j+dj-1) as usize;
            }
        }
        return count;
    }

    /// Return the position in the vector for the (i, j) coords of the cell.
    fn pos(&self, i: usize, j: usize) -> usize {
        return self.dims.0*i + j;
    }

    /// Return the GOL as a json-string.
    pub fn to_string(&self) -> String {
        json!(self).to_string()
    }
}

impl World for GameOfLife {

    fn update(&self, _: String) {
        // no-op
    }

    /// Update the gameboard with a new generation.
    /// NOTE: FOR DEMO ONLY. This is done inefficiently, but we're just try to get something that has a
    /// non-trivial gamestate to share.
    fn tick(&mut self) {

        let mut counts = Vec::<usize>::with_capacity(self.dims.0*self.dims.1);
        counts.resize_with(self.dims.0*self.dims.1, Default::default);

        // Create counts vector
        for i in 0..self.dims.0 {
            for j in 0..self.dims.1 {
                counts[self.pos(i, j)] = self.count(i, j);
            }
        }

        // Modify grid
        for i in 0..self.dims.0 {
            for j in 0..self.dims.1 {
                let pos = self.pos(i, j);
                let alive = self.grid[pos];
                let c = counts[pos];

                if alive {
                    if c < 2 || c > 3 {
                        self.grid[pos] = false;
                    } else {
                        self.grid[pos] = true;
                    }
                } else {
                    if c == 3 {
                        self.grid[pos] = true;
                    }
                }
            }
        }
    }
}

/// World is an initial janky implementation. The likely best trait just exposes a transmitter mpsc
/// object so that upstream systems can async publish things.
pub trait World {
    /// Update with some information from the outside world.
    fn update(&self, _: String);

    /// 
    fn tick(&mut self);
}
