use ::sdl::Rect;

pub trait Drawable {
    fn get_rect(&self) -> Rect;
}
