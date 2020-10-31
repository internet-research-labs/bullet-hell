// 
use std::fmt;

pub struct Pos {
    pub x: f32,
    pub y: f32,
}

pub struct Coord {
    theta: f32,
    fi: f32,
}

pub trait Coordinate {
    fn coord(&self) -> Coord;
}

impl Coordinate for Pos {
    fn coord(&self) -> Coord {
        return Coord{
            theta: 2.*self.x,
            fi: -3.*self.y,
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "Coord[{}, {}]",
            self.theta,
            self.fi,
        ))
    }
}
