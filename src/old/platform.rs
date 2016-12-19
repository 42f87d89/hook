use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;

use drawable::Drawable;
use tickable::Tickable;

pub struct Platform<'a> {
    surface: Surface<'a>,
    rect: Rect,
}

impl<'a> Platform<'a> {
    pub fn new(x: i32, y: i32) -> Self {
        let mut surface = Surface::new(20, 10, PixelFormatEnum::ARGB8888).unwrap();
        let _ = surface.fill_rect(None, Color::RGB(0, 0, 255));
        Platform {surface: surface, rect: Rect::new(x, y, 20, 10)}
    }
}

impl<'a> Tickable for Platform<'a> {
    fn tick(&mut self) {}
}

impl<'a> Drawable for Platform<'a> {
    fn get_surface(&self) -> &Surface {
        &self.surface
    }
}
