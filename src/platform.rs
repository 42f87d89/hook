use ::sdl::Rect;
use drawable::Drawable;
use tickable::Tickable;

pub struct Platform {
    rect: Rect,
}

impl Platform {
    pub fn new(x: i16, y: i16) -> Self {
        Platform {rect: Rect {x: x, y: y, w: 20, h: 10}}
    }
}

impl Tickable for Platform {
    fn tick(&mut self) {}
}

impl Drawable for Platform {
    fn get_rect(&self) -> Rect {
        self.rect
    }
}
