use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Vect {
    pub x: f64,
    pub y: f64,
}

impl Vect {
    pub fn size(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn norm(&self) -> Vect {
        if self.x == 0. && self.y == 0. {
            Vect {x: 0., y: 0.}
        } else {
            (1./self.size())*(*self)
        }
    }
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

impl Mul<Vect> for f64 {
    type Output = Vect;
    fn mul(self, rhs: Vect) -> Vect {
        Vect {x: self*rhs.x, y: self*rhs.y}
    }
}