use assets::alert::Alert;
use assets::button::Button;
use assets::fonts::Fonts;
use assets::game_data::GameData;
use assets::images::Images;
use assets::tileset::Tileset;
use std::rc::Rc;
use tetra::Context;

pub mod alert;
pub mod button;
pub mod data_entity;
pub mod fonts;
pub mod game_data;
pub mod images;
pub mod names;
pub mod prepared_font;
pub mod tileset;

pub struct Assets {
    pub fonts: Fonts, // no need Rc<> because they are cloned one by one
    pub images: Images,
    pub tileset: Rc<Tileset>,
    pub button: Rc<Button>,
    pub alert: Rc<Alert>,
    pub game_data: Rc<GameData>,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            fonts: Fonts::load(ctx)?,
            images: Images::load(ctx)?,
            tileset: Rc::new(Tileset::load(ctx)?),
            button: Rc::new(Button::load(ctx)?),
            alert: Rc::new(Alert::load(ctx)?),
            game_data: Rc::new(GameData::load()?),
        })
    }
}
