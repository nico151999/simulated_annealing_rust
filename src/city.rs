use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct City {
    x: f64,
    y: f64
}

impl City {
    pub fn new(x: f64, y: f64) -> City {
        City{x, y}
    }
    pub fn distance_to(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl PartialEq for City {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
