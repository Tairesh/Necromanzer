use tetra::{graphics::Texture, Context};

#[derive(Debug)]
pub struct Images {
    pub logo: Texture,
    pub bg: Texture,
    pub halloween: Texture,
}

impl Images {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            logo: Texture::from_encoded(ctx, include_bytes!("../../assets/img/logo.png"))?,
            bg: Texture::from_encoded(ctx, include_bytes!("../../assets/img/bg.jpg"))?,
            halloween: Texture::from_encoded(
                ctx,
                include_bytes!("../../assets/img/halloween.jpg"),
            )?,
        })
    }
}
