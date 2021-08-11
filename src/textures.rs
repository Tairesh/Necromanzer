use sdl2::gfx::primitives::ToColor;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct TextureManager {
    font_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new(
        font_context: Sdl2TtfContext,
        texture_creator: TextureCreator<WindowContext>,
    ) -> TextureManager {
        TextureManager {
            font_context,
            texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn load_image(&mut self, path: &str) -> &Texture {
        if !self.textures.contains_key(path) {
            self.textures.insert(
                path.to_string(),
                self.texture_creator.load_texture(path).unwrap(),
            );
        }
        self.textures.get(path).unwrap()
    }

    pub fn render_text(&mut self, text: &str, color: Option<Color>) -> &Texture {
        let default_font = self
            .font_context
            .load_font("res/fonts/consolab.ttf", 16)
            .unwrap();
        let color = color.unwrap_or(Color::RGB(0, 0, 0));
        let hash = format!("{}:{}", text, color.as_u32());
        if !self.textures.contains_key(hash.as_str()) {
            self.textures.insert(
                hash.clone(),
                self.texture_creator
                    .create_texture_from_surface(default_font.render(text).blended(color).unwrap())
                    .unwrap(),
            );
        }
        self.textures.get(hash.as_str()).unwrap()
    }
}
