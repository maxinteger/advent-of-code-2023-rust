use parse_display::FromStr;
use std::fmt::Display;

#[derive(FromStr, Eq, Hash, PartialEq, Clone, Copy, Default, Debug)]
#[display("{x},{y}")]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn distance(&self, other: &Self) -> isize {
        let x = other.x - self.x;
        let y = other.y - self.y;
        ((x * x + y * y) as f64).sqrt().floor() as isize
    }

    pub fn step_distance(&self, other: &Self) -> isize {
        let x = other.x - self.x;
        let y = other.y - self.y;
        x.abs() + y.abs()
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
