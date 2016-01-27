use platform::Platform;
use drawable::Drawable;

pub struct World<'a> {
    things: Vec<&'a Drawable>
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        World {things: Vec::new()}
    }
    pub fn add_thing(&mut self, thing: &'a Drawable) {
        self.things.push(thing);
    }
    pub fn get_things(&self) -> Vec<&'a Drawable> {
        self.things
    }
}
