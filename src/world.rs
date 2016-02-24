use tickable::Tickable;
use player::Player;

pub struct World<'a> {
    things: Vec<&'a mut Tickable>,
}

impl<'a> World<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        World {things: Vec::new()}
    }
    pub fn tick(&mut self) {
        for t in &mut self.things {
            t.tick();
        }
    }
    pub fn add_thing(&mut self, thing: &'a mut Tickable) {
        self.things.push(thing);
    }
    pub fn get_things(&self) -> &Vec<&'a mut Tickable> {
        &self.things
    }
}
