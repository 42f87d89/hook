extern crate sdl2;

mod screen;
mod input;
mod tickable;
mod drawable;
mod dot;
mod vect;
mod player;
mod platform;
mod world;

use screen::Screen;

use std::path::Path;
use sdl2::surface::Surface;

fn main() {
    let (width, height) = (800, 500);
    let ctx = sdl2::init().unwrap();
    
    let mut screen = Screen::init(width, height, &ctx);
    screen.draw(vec![match Surface::load_bmp(&Path::new("assets/Wood.bmp")) {
			Ok(surface) => surface,
			Err(err)    => panic!("failed to load surface")
		}]);
    screen.dothethings(ctx);
}