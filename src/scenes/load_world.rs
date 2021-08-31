use assets::Assets;
use chrono::{DateTime, Local};
use colors::Colors;
use savefile::{delete, savefiles, SaveFile};
use scenes::create_character::CreateCharacter;
use scenes::game::Game;
use scenes::manager::{update_sprites, Scene, Transition};
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
use tetra::{input, Context, TetraVec2};
use world::World;
use CARGO_VERSION;

pub struct LoadWorld {
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl LoadWorld {
    pub fn new(assets: Rc<RefCell<Assets>>, ctx: &mut Context) -> Self {
        let savefiles = savefiles();
        let mut sprites: Vec<Rc<RefCell<dyn Sprite>>> = Vec::with_capacity(savefiles.len() * 6 + 1);
        let height = savefiles.len() as f32 * 50.0 + 33.0;
        let mut y = -height / 2.0;
        sprites.push(Rc::new(RefCell::new(Alert::new(
            600.0,
            height,
            assets.clone(),
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
                TetraVec2::new(560.0, 50.0),
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -282.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                savefile.meta.name.as_str(),
                assets.borrow().header2.clone(),
                Colors::LIGHT_YELLOW,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -280.0 },
                    y: Vertical::AtWindowCenterByTop { offset: y - 2.0 },
                },
            ))));
            sprites.push(Rc::new(RefCell::new(Label::new(
                savefile.version.as_str(),
                assets.borrow().default.clone(),
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
            let time: DateTime<Local> = savefile.time.into();
            sprites.push(Rc::new(RefCell::new(Label::new(
                time.format("%Y.%m.%d %H:%M:%S").to_string().as_str(),
                assets.borrow().default.clone(),
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
        LoadWorld { assets, sprites }
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
                    Transition::Replace(Box::new(LoadWorld::new(self.assets.clone(), ctx)))
                })
            }
            (Some("load"), Some(path)) => {
                let savefile = SaveFile::load(path.parse().unwrap()).unwrap();
                Some(if savefile.avatar_data.is_empty() {
                    Transition::Replace(Box::new(CreateCharacter::new(
                        self.assets.clone(),
                        savefile,
                        ctx,
                    )))
                } else {
                    Transition::Replace(Box::new(Game::new(
                        self.assets.clone(),
                        World::new(
                            savefile.path.clone(),
                            savefile.meta.clone(),
                            savefile.load_avatar(),
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
