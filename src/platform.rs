use ::sdl::Rect;
use drawable::Drawable;

pub struct Platform {
    rect: Rect,
}

impl Platform {
    pub fn new(x: i16, y: i16) -> Self {
        Platform {rect: Rect {x: x, y: y, w: 10, h: 10}}
    }
}

impl Drawable for Platform {
    fn get_rect(&self) -> Rect {
        self.rect
    }
}
