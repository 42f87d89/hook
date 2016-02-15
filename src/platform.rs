use drawable::Drawable;
use tickable::Tickable;

pub struct Platform<'a> {
    rect: Rect,
}

impl<'a> Platform<'a> {
    pub fn new(x: i16, y: i16) -> Self {
        Platform {rect: Rect {x: x, y: y, w: 20, h: 10}}
    }
}

impl<'a> Tickable for Platform<'a> {
    fn tick(&mut self) {}
}

impl<'a> Drawable for Platform<'a> {
    fn get_surface(&self) -> Surface {
        self.surface
    }
}
