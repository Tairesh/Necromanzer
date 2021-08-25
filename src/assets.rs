use tetra::graphics::text::Font;
use tetra::graphics::Texture;
use tetra::Context;

pub struct Assets {
    pub consolab: Font,
    pub avqest: Font,
    pub logo: Texture,
    pub bg: Texture,
    pub button: Texture,
    pub button_disabled: Texture,
    pub button_pressed: Texture,
    pub button_hovered: Texture,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Assets {
            consolab: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/consolab.ttf"),
                16.0,
            )?,
            avqest: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/avqest.ttf"),
                76.0,
            )?,
            logo: Texture::from_file_data(ctx, include_bytes!("../res/img/logo.png"))?,
            bg: Texture::from_file_data(ctx, include_bytes!("../res/img/bg.jpg"))?,
            button: Texture::from_file_data(ctx, include_bytes!("../res/img/button.png"))?,
            button_disabled: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_disabled.png"),
            )?,
            button_pressed: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_pressed.png"),
            )?,
            button_hovered: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_hovered.png"),
            )?,
        })
    }
}
