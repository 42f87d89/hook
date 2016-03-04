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
use player::Player;
use drawable::Drawable;
use tickable::Tickable;

use std::cell::Cell;
use std::cell::RefCell;
use std::time::Duration;
use std::thread;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let (width, height) = (800, 500);
    let ctx = sdl2::init().unwrap();
    let mut screen = Screen::init(width, height, &ctx);
    let should_end = Cell::new(false);
    let player = RefCell::new(Player::new(20, 20));
    let mut input = Input::new(ctx);

    input.add_handler(Box::new(|evt| {
        let player = player.borrow_mut();
        match evt {
            &Event::KeyDown {keycode: Some(keycode), ..} => {
                let mut ver = 0;
                let mut hor = 0;
                if keycode == Keycode::W {
                    ver += -1;
                } else if keycode == Keycode::S {
                    ver += 1;
                } else if keycode == Keycode::A {
                    hor += -1;
                } else if keycode == Keycode::D {
                    hor += 1;
                }
                player.hor.set(hor);
                player.ver.set(ver);
            }
            &Event::KeyUp {keycode: Some(keycode), ..} => { // Might cause 1 frame of lag when letting go of one direction, then immediately pressing another
                if keycode == Keycode::W || keycode == Keycode::S {
                    player.ver.set(0);
                } else if keycode == Keycode::A || keycode == Keycode::D {
                    player.hor.set(0);
                }
            }
            _ => {}
        }
    }));
        
    input.add_handler(Box::new(|evt| {
        match evt {
            &Event::Quit{..} => {
                should_end.set(true);
            }
            &Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                should_end.set(true);
            }
            _ => {}
        }
    }));
    while !should_end.get() {
        input.handle();
        player.borrow_mut().tick();
        screen.draw(vec![(player.borrow().rect.x(), player.borrow().rect.y(), player.borrow().get_surface())]);
        thread::sleep(Duration::new(0, 1000000000/60));
    }
}