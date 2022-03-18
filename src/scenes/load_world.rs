use assets::Assets;
use colors::Colors;
use savefile::{delete, savefiles, SaveFile};
use scenes::create_character::CreateCharacter;
use scenes::game::scene::Game;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::alert::Alert;
use sprites::button::Button;
use sprites::label::Label;
use sprites::meshy::HoverableMesh;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{Color, Rectangle};
use tetra::input::{Key, MouseButton};
use tetra::{input, Context};
use time::format_description::FormatItem;
use time::OffsetDateTime;
use world::World;
use {Vec2, CARGO_VERSION};

const DATETIME_FORMAT: &[FormatItem] =
    time::macros::format_description!("[year].[month].[day] [hour]:[minute]:[second]");

pub struct LoadWorld {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl LoadWorld {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        ctx: &mut Context,
    ) -> Self {
        let savefiles = savefiles();
        let mut sprites: Vec<Rc<RefCell<dyn Sprite>>> = Vec::with_capacity(savefiles.len() * 6 + 1);
        let height = savefiles.len() as f32 * 50.0 + 33.0;
        let mut y = -height / 2.0;
        sprites.push(Rc::new(RefCell::new(Alert::new(
            600.0,
            height,
            assets.borrow().alert_asset.texture.clone(),
            assets.borrow().alert_asset.nineslice.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByTop { offset: y - 18.0 },
            },
        ))));
        for (i, savefile) in savefiles.iter().enumerate() {
            sprites.push(Rc::new(RefCell::new(HoverableMesh::new(
                Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 564.0, 50.0))
                    .unwrap(),
                if i % 2 == 1 {
                    Colors::DARK_GRAY.with_alpha(0.3)
                } else {
                    Colors::TRANSPARENT
                },
                Colors::KHAKI.with_alpha(0.6),
                Vec2::new(560.0, 50.0),
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -282.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                savefile.meta.name.as_str(),
                assets.borrow().fonts.header2.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -280.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y - 2.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                savefile.version.as_str(),
                assets.borrow().fonts.default.clone(),
                if savefile.version.as_str() == CARGO_VERSION {
                    Colors::GREEN
                } else {
                    Color::RED
                },
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -275.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y + 30.0 },
                },
            ))));
            let time =
                OffsetDateTime::from(savefile.time).to_offset(settings.borrow().offset.unwrap());
            sprites.push(Rc::new(RefCell::new(Label::new(
                time.format(&DATETIME_FORMAT).unwrap(),
                assets.borrow().fonts.default.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -220.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y + 30.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Button::new(
                format!("load:{}", savefile.path.to_str().unwrap()).as_str(),
                vec![],
                "Load",
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByRight { offset: 190.0 },
                    y: Vertical::AtWindowCenter { offset: y + 24.5 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Button::new(
                format!("del:{}", savefile.path.to_str().unwrap()).as_str(),
                vec![],
                "Delete",
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByRight { offset: 275.0 },
                    y: Vertical::AtWindowCenter { offset: y + 24.5 },
                },
            ))));
            y += 50.0;
        }
        LoadWorld {
            assets,
            settings,
            sprites,
        }
    }
}

impl Scene for LoadWorld {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        let mut parts = btn_id.split(':');
        match (parts.next(), parts.next()) {
            (Some("back"), _) => Some(Transition::Pop),
            (Some("del"), Some(path)) => {
                let path = path.parse::<PathBuf>().unwrap();
                delete(&path);
                Some(if savefiles().is_empty() {
                    Transition::Pop
                } else {
                    Transition::Replace(Box::new(LoadWorld::new(
                        self.assets.clone(),
                        self.settings.clone(),
                        ctx,
                    )))
                })
            }
            (Some("load"), Some(path)) => {
                let savefile = SaveFile::load(path.parse().unwrap()).unwrap();
                Some(if savefile.units_data.is_empty() {
                    Transition::Replace(Box::new(CreateCharacter::new(
                        self.assets.clone(),
                        self.settings.clone(),
                        savefile,
                        ctx,
                    )))
                } else {
                    Transition::Replace(Box::new(Game::new(
                        self.assets.clone(),
                        self.settings.clone(),
                        World::new(
                            self.assets.clone(),
                            savefile.path.clone(),
                            savefile.meta.clone(),
                            savefile.load_avatar(),
                            savefile.load_chunks(),
                        ),
                        ctx,
                    )))
                })
            }
            _ => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1)
            || input::is_key_pressed(ctx, Key::Escape)
        {
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
