use sdl2::rect::Rect;
use sdl2::surface::Surface;
use std::path::Path;

use drawable::Drawable;
use tickable::Tickable;
use dot::Dot;
use vect::Vect;

pub struct Player<'a> {
    surface: Surface<'a>,
    rect: Rect,
	dot: Dot,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl<'a> Player<'a> {
    pub fn new(x: i32, y: i32) -> Self {
        let surface = match Surface::load_bmp(&Path::new("assets/Wood.bmp")) {
			Ok(surface) => surface,
			Err(err)    => panic!("failed to load surface")
		};
        Player {
            surface: surface,
            rect: Rect::new(x, y, 20, 80),
            dot: Dot::new(x as f64, y as f64),
            up: false, down: false, left: false, right: false
        }
    }
}

impl<'a> Tickable for Player<'a> {
    fn tick(&mut self) {
        let mut acc = Vect {x:0., y: 0.};
        if self.up && !self.down {
            acc.y = -0.01;
        } else if self.down {
            acc.y = 0.01;
        } else {
            acc.y = 0.;
        }

        if self.left && !self.right {
            acc.x = -0.01;
        } else if self.right {
            acc.x = 0.01;
        } else {
            acc.x = 0.;
        }
        self.dot.set_force(acc);
        self.dot.accelerate();
        self.dot.move_it();
        
        self.dot.cap_speed(3.0);
        self.dot.apply_friction(1.0);

		self.rect.set_x(self.dot.get_pos().x as i32);
		self.rect.set_y(self.dot.get_pos().y as i32);
    }
}

impl<'a> Drawable for Player<'a> {
    fn get_surface(&self) -> &Surface {
        &self.surface
    }
}
