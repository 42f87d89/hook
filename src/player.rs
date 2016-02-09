use drawable::Drawable;
use tickable::Tickable;
use ::sdl::Rect;

pub struct Player {
    rect: Rect,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Player {
    pub fn new(rect: Rect) -> Self {
        Player {rect: rect, up: false, down: false, left: false, right: false}
    }    
}

impl Tickable for Player {
    fn tick(&mut self) {
        if self.up {
            self.rect.y -= 1;
        }
        if self.down {
            self.rect.y += 1;
        }
        if self.left {
            self.rect.x -= 1;
        }
        if self.right {
            self.rect.x += 1;
        }
    }
}

impl Drawable for Player {
    fn get_rect(&self) -> Rect {
        self.rect
    }
}
