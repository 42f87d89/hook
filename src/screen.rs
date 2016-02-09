use ::sdl::video;
use ::sdl::Rect;
use ::sdl::video::{Surface, SurfaceFlag, VideoFlag, Color};
use ::sdl::InitFlag::{Video};
use ::sdl::wm;
use drawable::Drawable;
use world::World;

pub struct Screen {
    width: isize,
    height: isize,
    surface: Surface,
    pub should_end: bool,
}

impl Screen {
    pub fn new(w: isize, h: isize) -> Self {
        ::sdl::init(&[Video]);
        wm::set_caption("String", "String");

        let s = match video::set_video_mode(w, h, 32,
                                                 &[SurfaceFlag::HWSurface],
                                                 &[VideoFlag::DoubleBuf]) {
            Ok(s) => s,
            Err(err) => panic!("failed to set video mode: {}", err)
        };
        Screen {width: w, height: h, surface: s, should_end: false}
    }
    pub fn should_end(&self) -> bool {
        self.should_end
    }
    fn draw_square(&self, rect: Rect, (r,g,b): (u8, u8, u8)) {
        self.surface.fill_rect(Some(rect), Color::RGB(r, g, b));
    }
    pub fn draw(&self, things: &World) {
        self.surface.flip();
        self.surface.fill(Color::RGB(0,0,0));
        self.draw_square(things.get_player().get_rect(), (255,0,255));
        for t in things.get_things() {
            self.draw_square(t.get_rect(), (255,255,255));
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        ::sdl::quit();
    }
}
