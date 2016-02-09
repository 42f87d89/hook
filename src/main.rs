extern crate sdl;

mod screen;
mod player;
mod world;
mod drawable;
mod platform;
mod input;
mod tickable;

use screen::Screen;
use player::Player;
use world::World;
use platform::Platform;
use ::sdl::Rect;

fn main() {
    let mut screen = Screen::new(800, 600);
    let mut player = Player::new(Rect {x: 20, y: 20, w: 10, h: 10});
    let mut plat1 = Platform::new(0, 0);
    let mut plat2 = Platform::new(50, 0);
    let mut plat3 = Platform::new(0, 50);
    let mut plat4 = Platform::new(50, 50);
    let mut world = World::new(&mut player);
    world.add_thing(&mut plat1);
    world.add_thing(&mut plat2);
    world.add_thing(&mut plat3);
    world.add_thing(&mut plat4);
    loop {
        let event = ::sdl::event::poll_event();
        input::screen_should_end(&event, &mut screen);
        world.tick(&event);
        if screen.should_end() {break;}
        screen.draw(&world);
    }
}
