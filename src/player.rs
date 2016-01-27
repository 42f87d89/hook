use drawable::Drawable;
use ::sdl::Rect;

pub struct Player {
    rect: Rect,
}

impl Player {
    pub fn new(rect: Rect) -> Self {
        Player {rect: rect}
    }    
}

impl Drawable for Player {
    fn get_rect(&self) -> Rect {
        self.rect
    }
}
