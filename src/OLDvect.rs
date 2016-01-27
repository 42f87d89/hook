use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
}

impl Add for Vect {
    type Output = Vect;
    fn add(self, vec: Vect) -> Vect {
        Vect {x: self.x + vec.x, y: self.y + vec.y}
    }
}

impl Sub for Vect {
    type Output = Vect;
    fn sub(self, vec: Vect) -> Vect {
        Vect {x: self.x - vec.x, y: self.y - vec.y}
    }
}

impl Vect {
    pub fn size(&self) -> f64 {
        (self.x*self.x + self.y* self.y).sqrt()
    }
    pub fn scale(&mut self, r: f64) {
        self.x *= r;
        self.y *= r;
    }
}
