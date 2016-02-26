use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::render::Renderer;
use sdl2::Sdl;
use std::path::Path;

pub struct Screen {
	renderer: Renderer<'static>,
}

impl Screen {
	pub fn init(width: u32, height: u32, ctx: &Sdl) -> Self {
		let video_ctx = ctx.video().unwrap();

		let window  = match video_ctx.window("Hook", width, height).position_centered().opengl().build() {
			Ok(window) => window,
			Err(err)   => panic!("failed to create window")
		};

		let mut renderer = match window.renderer().build() {
			Ok(renderer) => renderer,
			Err(err) => panic!("failed to create renderer")
		};

		Screen {renderer: renderer}		
	}
	pub fn draw(&mut self, surfaces: Vec<Surface>) {
		for surface in surfaces {
			// Convert a surface to a texture.
			// Textures can be used more efficiently by the GPU. (If one is available.)
			let texture = match self.renderer.create_texture_from_surface(&surface) {
				Ok(texture) => texture,
				Err(err)    => panic!("failed to convert surface")
			};

			let _ = self.renderer.clear();
			// Display the texture.
			// Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
			// Try passing Some(surface.get_rect()) for src & dst instead of None & see how things change.
			let _ = self.renderer.copy(&texture, None, None);
			let _ = self.renderer.present();
		}
	}
}