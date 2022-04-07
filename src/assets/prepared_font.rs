use tetra::graphics::text::{Font, Text};
use tetra::Context;

#[derive(Debug, Clone)]
pub struct PreparedFont {
    pub font: Font,
    pub line_height: f32,
}

impl PreparedFont {
    pub fn new(ctx: &mut Context, font: Font) -> tetra::Result<Self> {
        // TODO: implement more adequate way to detect tallest symbol
        let bounds = Text::new("IjqgpT})@", font.clone())
            .get_bounds(ctx)
            .unwrap();
        Ok(Self {
            font,
            line_height: bounds.height,
        })
    }
}
