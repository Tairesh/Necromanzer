use std::rc::Rc;

use tetra::Context;

use assets::alert::Alert;
use assets::button::Button;
use assets::fonts::Fonts;
use assets::images::Images;
use assets::tileset::Tileset;

pub mod alert;
pub mod button;
pub mod fonts;
pub mod images;
pub mod names;
pub mod prepared_font;
pub mod tileset;

// Can't put this to OnceCell because tetra::Font and tetra::Texture uses Rc<> inside
pub struct Assets {
    pub fonts: Fonts,
    pub images: Images,
    pub tileset: Rc<Tileset>,
    pub button: Rc<Button>,
    pub alert: Rc<Alert>,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            fonts: Fonts::load(ctx)?,
            images: Images::load(ctx)?,
            tileset: Rc::new(Tileset::load(ctx)?),
            button: Rc::new(Button::load(ctx)?),
            alert: Rc::new(Alert::load(ctx)?),
        })
    }
}
