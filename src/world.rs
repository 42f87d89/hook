use drawable::Drawable;
use tickable::Tickable;
use player::Player;
use input;
use ::sdl::event::Event;

trait DTAble: Tickable + Drawable {}
impl<T> DTAble for T where T: Tickable + Drawable {}

pub struct World<'a> {
    things: Vec<&'a mut DTAble>,
    player: &'a mut Player
}

impl<'a> World<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        World {things: Vec::new(), player: player}
    }
    pub fn tick(&mut self, event: &Event) {
        input::move_player(event, &mut self.player);
        for t in &mut self.things {
            t.tick();
        }
    }
    pub fn add_thing(&mut self, thing: &'a mut DTAble) {
        self.things.push(thing);
    }
    pub fn get_things(&self) -> &Vec<&'a mut DTAble> {
        &self.things
    }
    pub fn get_player(&self) -> &Player {
        self.player
    }
}
