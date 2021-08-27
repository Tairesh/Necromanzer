use tetra::graphics::text::Font;
use tetra::graphics::Texture;
use tetra::Context;

pub struct Assets {
    pub default: Font,
    pub header1: Font,
    pub header2: Font,
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
            default: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/consolab.ttf"),
                16.0,
            )?,
            header1: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/avqest.ttf"),
                86.0,
            )?,
            header2: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/avqest.ttf"),
                32.0,
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
