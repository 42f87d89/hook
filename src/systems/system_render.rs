use ::sdl2::surface::Surface;
use ::sdl2::pixels::PixelFormatEnum;
use ::sdl2::pixels::Color::RGB;
use ::sdl2::render::Renderer;
use ::specs::RunArg;
use ::specs::Join;

use ::components::component_render::ComponentRender;

pub fn system_render(arg: RunArg, mut renderer: Renderer<'static>) {
    // fetch() borrows a world, so a system could lock necessary storages
    // Can be called only once
    let storage_render = arg.fetch(|w| {w.read::<ComponentRender>()});
    for cr in (&storage_render).iter() {
        let mut surface = Surface::new(cr.w as u32, cr.h as u32, PixelFormatEnum::RGB24).unwrap();
        let _ = surface.fill_rect(None, RGB(255,0,0));
        // Convert a surface to a texture.
        // Textures can be used more efficiently by the GPU. (If one is available.)
        let texture = match renderer.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(_)    => panic!("failed to convert surface")
        };

        let _ = renderer.clear();
        // Display the texture.
        // Omitting the src & dst Rect arguments will cause our image to stretch across the entire buffer.
        // Try passing Some(surface.get_rect()) for src & dst instead of None & see how things change.
        let mut rect = surface.rect();
        rect.set_x(cr.x);
        rect.set_y(cr.y);
        let _ = renderer.copy(&texture, None, Some(rect));
        let _ = renderer.present();
    }
}