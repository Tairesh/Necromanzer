use tetra::graphics::{NineSlice, Rectangle, Texture};
use tetra::Context;

#[derive(Debug, Clone)]
pub struct Alert {
    pub texture: Texture,
    pub nineslice: NineSlice,
}

impl Alert {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            texture: Texture::from_encoded(ctx, include_bytes!("../../inc/img/alert.png"))?,
            nineslice: NineSlice::new(Rectangle::new(0.0, 0.0, 48.0, 32.0), 6.0, 6.0, 6.0, 5.0),
        })
    }
}
