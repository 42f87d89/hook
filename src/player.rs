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
    pub up: Cell<bool>,
    pub down: Cell<bool>,
    pub left: Cell<bool>,
    pub right: Cell<bool>,
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
            up: Cell::new(false),
            down: Cell::new(false),
            left: Cell::new(false),
            right: Cell::new(false)
        }
    }
}

impl<'a> Tickable for Player<'a> {
    fn tick(&mut self) {
        let acc1 = 0.5;
        let acc = Vect {
            x:if self.left.get() && !self.right.get() {
                -acc1
            } else if self.right.get() {
                acc1
            } else {
                0.
            },
            y: if self.up.get() && !self.down.get() {
                -acc1
            } else if self.down.get() {
                acc1
            } else {
                0.
            }
        };
        self.dot.tick(acc, 5.0);

		self.rect.set_x(self.dot.get_pos().x as i32);
		self.rect.set_y(self.dot.get_pos().y as i32);
    }
}

impl<'a> Drawable for Player<'a> {
    fn get_surface(&self) -> &Surface {
        &self.surface
    }
}
