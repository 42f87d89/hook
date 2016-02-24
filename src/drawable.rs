use sdl2::surface::Surface;

pub trait Drawable {
    fn get_surface(&self) -> &Surface;
}
