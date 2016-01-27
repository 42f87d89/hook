use vect::Vect;

#[derive(Clone, Copy)]
pub struct Dot {
    pos: Vect,
    vel: Vect,
    acc: Vect,
}

impl Dot {
    pub fn new(x: f64, y: f64) -> Dot {
        Dot {
            pos: Vect {x: x, y: y},
            vel: Vect {x: 0., y: 0.},
            acc: Vect {x: 0., y: 0.},
        }
    }
    pub fn move_it(&mut self) {
        self.pos = self.pos + self.vel;
    }
    pub fn accelerate(&mut self) {
        self.vel = self.vel + self.acc;
    }
    pub fn get_force(self, dot: Dot) -> Vect {
        let x = dot.pos.x - self.pos.x;
        let y = dot.pos.y - self.pos.y;
        Vect {x: x, y: y}
    }
    pub fn set_force(&mut self, vect: Vect) {
        self.acc = vect;
    }
}
