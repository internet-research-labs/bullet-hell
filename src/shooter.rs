use crate::conceptual;

use std::collections::HashMap;
use std::time;


#[derive(Clone)]
pub struct Player {
    id: usize,
    x: isize,
    y: isize,
}

impl Player {
    pub fn to_string(&self) -> String {
        format!(
            "{},{},{},-1,-1",
            10, self.x, self.y,
        ).to_string()
    }
}

pub struct Shooter {
    h: usize,
    w: usize,
    ships: HashMap<usize, Player>,
    timestamp: time::SystemTime,
}


impl Shooter {
}


impl conceptual::World for Shooter {

    fn update(&self, _: String) {
    }

    fn tick(&mut self) {
        if let Some(ship) = self.ships.get_mut(&10) {
            ship.y = if ship.y < 100 {
                ship.y + 1
            } else {
                -100
            };
        }
        self.timestamp = time::SystemTime::now();
    }

    fn to_string(&self) -> String {

        let mut s = String::from("");
        
        if let Ok(dur) = self.timestamp.duration_since(time::UNIX_EPOCH) {
            s.push_str(&dur.as_millis().to_string());
            s.push_str("::");
        } else {
            s.push_str("...");
        }

        if let Some(ship) = self.ships.get(&10) {
            s.push_str(&ship.to_string())
        }

        s
    }

    fn reset(&mut self) {
    }
}

pub fn with_size(h: usize, w: usize) -> Shooter {
    let id = 10;
    let mut ships = HashMap::new();
    ships.insert(
        id,
        Player{id: id, x: 0, y: 0},
    );
    Shooter{
        h: h,
        w: w,
        ships: ships,
        timestamp: time::SystemTime::now(),
    }
}
