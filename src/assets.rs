use std::collections::HashMap;
use tetra::graphics::text::Font;
use tetra::graphics::Texture;
use tetra::Context;

type TextureHashmap = HashMap<TextureId, Texture>;
type FontHashmap = HashMap<FontId, Font>;

pub struct Assets {
    textures: TextureHashmap,
    fonts: FontHashmap,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Assets {
            textures: build_textures(ctx)?,
            fonts: build_fonts(ctx)?,
        })
    }

    pub fn get_texture(&self, id: TextureId) -> &Texture {
        &self.textures[&id]
    }

    pub fn get_font(&self, id: FontId) -> &Font {
        &self.fonts[&id]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureId {
    Logo,
    Background,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FontId {
    Default,
    Avqest,
}

fn build_textures(ctx: &mut Context) -> tetra::Result<TextureHashmap> {
    Ok([
        (
            TextureId::Logo,
            Texture::from_file_data(ctx, include_bytes!("../res/img/logo.png"))?,
        ),
        (
            TextureId::Background,
            Texture::from_file_data(ctx, include_bytes!("../res/img/bg.jpg"))?,
        ),
    ]
    .iter()
    .cloned()
    .collect())
}

fn build_fonts(ctx: &mut Context) -> tetra::Result<FontHashmap> {
    Ok([
        (
            FontId::Default,
            Font::from_vector_file_data(ctx, include_bytes!("../res/fonts/consolab.ttf"), 16.0)?,
        ),
        (
            FontId::Avqest,
            Font::from_vector_file_data(ctx, include_bytes!("../res/fonts/avqest.ttf"), 20.0)?,
        ),
    ]
    .iter()
    .cloned()
    .collect())
}
