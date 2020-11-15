use crate::conceptual;

use std::collections::HashMap;


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
    }

    fn to_string(&self) -> String {
        if let Some(ship) = self.ships.get(&10) {
            ship.to_string()
        } else {
            "".to_string()
        }
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
    }
}
