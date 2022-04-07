use assets::alert::Alert;
use assets::button::Button;
use assets::fonts::Fonts;
use assets::images::Images;
use assets::names::Names;
use assets::tileset::Tileset;
use tetra::Context;

pub mod alert;
pub mod button;
pub mod fonts;
pub mod images;
pub mod names;
pub mod prepared_font;
pub mod tileset;

pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub tileset: Tileset,
    pub names: Names,
    pub button: Button,
    pub alert: Alert,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            fonts: Fonts::load(ctx)?,
            images: Images::load(ctx)?,
            tileset: Tileset::load(ctx)?,
            names: Names::load()?,
            button: Button::load(ctx)?,
            alert: Alert::load(ctx)?,
        })
    }
}
