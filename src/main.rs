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
use input::Input;

use std::path::Path;
use sdl2::surface::Surface;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let (width, height) = (800, 500);
    let ctx = sdl2::init().unwrap();

    let mut screen = Screen::init(width, height, &ctx);
    let mut should_end = false;
    let mut input = Input::new(ctx);
    input.add_handler(Box::new(|evt| {
        match evt {
            &Event::Quit{..} => {
                should_end = true;
            }
            &Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                should_end = true;
            }
            _ => {}
        }
    }));
    while !should_end {
        input.handle();
    }
}