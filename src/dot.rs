use vect::Vect;

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
    pub fn tick(&mut self, acc: Vect, max_speed: f64) {
        self.set_force(acc);
        self.accelerate();
        self.cap_speed(max_speed);
        self.move_it();
    }
    fn move_it(&mut self) {
        self.pos = self.pos + self.vel;
    }
    fn accelerate(&mut self) { // Diagonal accelerate faster
        self.vel = self.vel + self.acc;
    }
    fn set_force(&mut self, vect: Vect) {
        self.acc = vect;
    }
    pub fn get_pos(&self) -> Vect {
        self.pos
    }
    fn cap_speed(&mut self, max: f64) {
        if self.vel.size() > max {
            self.vel = max*self.vel.norm();
        }
    }
}
