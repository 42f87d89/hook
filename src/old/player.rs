use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::cell::Cell;

use drawable::Drawable;
use tickable::Tickable;
use dot::Dot;
use vect::Vect;
use input::Input;

pub struct Player<'a> {
    surface: Surface<'a>,
    pub rect: Rect,
	dot: Dot,
    pub ver: Cell<i8>,
    pub hor: Cell<i8>,
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
            ver: Cell::new(0),
            hor: Cell::new(0)
        }
    }
}

impl<'a> Tickable for Player<'a> {
    fn tick(&mut self) {
        let acc_scale = 0.5;
        let fric_scale = -0.2;
        let max_speed = 5.0;
        
        let mut acc = acc_scale*Vect {x: self.hor.get() as f64, y: self.ver.get() as f64}.norm();
        let fric = fric_scale*self.dot.get_vel().norm();
        self.dot.tick(acc+fric, max_speed);
        
		self.rect.set_x(self.dot.get_pos().x.round() as i32);
		self.rect.set_y(self.dot.get_pos().y.round() as i32);
    }
}

impl<'a> Drawable for Player<'a> {
    fn get_surface(&self) -> &Surface {
        &self.surface
    }
}
