extern crate find_folder;
extern crate texture;

use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::RenderArgs;

pub struct Display {}

impl Display {
    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs ) {
        use graphics::*;

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");
        let mut glyphs = GlyphCache::new(font, 
            (),
            texture::TextureSettings::new()).unwrap();
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform.trans(0.0, 32.0);
            text::Text::new_color([1.0, 0.0, 0.0, 1.0], 32).draw(
                "1",
                &mut glyphs,
                &c.draw_state,
                transform, gl
            )}
        ).unwrap();
    }
}