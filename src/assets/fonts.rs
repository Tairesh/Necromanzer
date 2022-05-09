use tetra::graphics::text::Font;
use tetra::Context;

use assets::prepared_font::PreparedFont;

#[derive(Debug)]
pub struct Fonts {
    pub default: PreparedFont,
    pub default2: PreparedFont,
    pub header1: PreparedFont,
    pub header2: PreparedFont,
}

impl Fonts {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        let consolab = include_bytes!("../../assets/fonts/consolab.ttf");
        let consolab16 = Font::from_vector_file_data(ctx, consolab, 16.0)?;
        let consolab24 = Font::from_vector_file_data(ctx, consolab, 24.0)?;
        let avqest = include_bytes!("../../assets/fonts/avqest.ttf");
        let avqest86 = Font::from_vector_file_data(ctx, avqest, 86.0)?;
        let avqest32 = Font::from_vector_file_data(ctx, avqest, 32.0)?;

        Ok(Self {
            default: PreparedFont::new(ctx, consolab16)?,
            default2: PreparedFont::new(ctx, consolab24)?,
            header1: PreparedFont::new(ctx, avqest86)?,
            header2: PreparedFont::new(ctx, avqest32)?,
        })
    }
}
