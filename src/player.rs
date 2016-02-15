use drawable::Drawable;
use tickable::Tickable;
use dot::Dot;
use vect::Vect;


pub struct Player<'a> {
    rect: Rect,
	dot: Dot,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl<'a> Player<'a> {
    pub fn new(rect: Rect) -> Self {
        Player {rect: rect, dot: Dot::new(rect.x as f64, rect.y as f64), up: false, down: false, left: false, right: false}
    }
}

impl<'a> Tickable for Player<'a> {
    fn tick(&mut self) {
        let mut acc = Vect {x:0., y: 0.};
        if self.up {
            acc.y = -0.01;
        } else if self.down {
            acc.y = 0.01;
        } else {
            acc.y = 0.;
        }
        
        if self.left {
            acc.x = -0.01;
        } else if self.right {
            acc.x = 0.01;
        } else {
            acc.x = 0.;
        }
        self.dot.set_force(acc);
        self.dot.accelerate();
        self.dot.move_it();
        self.dot.apply_friction(1.0);
        
		self.rect.x = self.dot.get_pos().x as i16;
		self.rect.y = self.dot.get_pos().y as i16;
    }
}

impl<'a> Drawable for Player<'a> {
    fn get_surface(&self) -> Surface {
        self.surface
    }
}
