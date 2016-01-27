extern crate sdl;

mod screen;
mod player;
mod world;
mod drawable;
mod platform;

use screen::Screen;
use player::Player;
use world::World;
use platform::Platform;
use ::sdl::Rect;

fn main() {
    let mut screen = Screen::new(800, 600);
    let mut world = World::new();
    let mut player = Player::new(Rect {x: 10, y: 10, w: 10, h: 10});
    world.add_thing(&mut player);
    world.add_thing(&Platform::new(0, 0));
    world.add_thing(&Platform::new(50, 0));
    world.add_thing(&Platform::new(50, 50));
    world.add_thing(&Platform::new(0, 50));
    loop {
        screen.tick();
        if screen.should_end() {break;}
        screen.draw(world);
    }
}
