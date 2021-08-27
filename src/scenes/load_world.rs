use assets::Assets;
use chrono::{DateTime, Local};
use colors::Colors;
use savefile::{delete, savefiles};
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::alert::Alert;
use sprites::button::Button;
use sprites::label::Label;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::{Positionate, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::Color;
use tetra::input::MouseButton;
use tetra::{input, Context};
use CARGO_VERSION;

pub struct LoadWorld {
    sprites: Vec<Box<dyn Sprite>>,
}

impl LoadWorld {
    pub fn new(assets: Rc<RefCell<Assets>>, ctx: &mut Context) -> Self {
        let savefiles = savefiles();
        let mut sprites: Vec<Box<dyn Sprite>> = Vec::with_capacity(savefiles.len() * 5 + 1);
        let height = savefiles.len() as f32 * 16.0 + 11.0;
        let mut y = -height * 1.5;
        sprites.push(Box::new(Alert::new(
            600.0 / 3.0,
            height,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByTop { offset: y - 30.0 },
            },
        )));
        for savefile in savefiles {
            let mut label = Label::new(
                savefile.name.as_str(),
                assets.borrow().header2.clone(),
                Colors::DARK_BROWN,
                Position {
                    x: Horizontal::AtWindowCenterByLeft { offset: -290.0 },
                    y: Vertical::AtWindowCenter { offset: y + 15.0 },
                },
            );
            let size = label.calc_size(ctx);
            sprites.push(Box::new(label));
            sprites.push(Box::new(Label::new(
                savefile.version.as_str(),
                assets.borrow().default.clone(),
                if savefile.version.as_str() == CARGO_VERSION {
                    Colors::GREEN
                } else {
                    Color::RED
                },
                Position {
                    x: Horizontal::AtWindowCenterByLeft {
                        offset: -260.0 + size.x,
                    },
                    y: Vertical::AtWindowCenter { offset: y + 20.0 },
                },
            )));
            let time: DateTime<Local> = savefile.time.into();
            sprites.push(Box::new(Label::new(
                time.format("%Y.%m.%d %H:%M:%S").to_string().as_str(),
                assets.borrow().default.clone(),
                Colors::DARK_BROWN,
                Position {
                    x: Horizontal::AtWindowCenterByLeft {
                        offset: -200.0 + size.x,
                    },
                    y: Vertical::AtWindowCenter { offset: y + 20.0 },
                },
            )));
            sprites.push(Box::new(Button::new(
                format!("load:{}", savefile.path.to_str().unwrap()).as_str(),
                vec![],
                "Load",
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByRight { offset: 200.0 },
                    y: Vertical::AtWindowCenter { offset: y + 25.0 },
                },
            )));
            sprites.push(Box::new(Button::new(
                format!("del:{}", savefile.path.to_str().unwrap()).as_str(),
                vec![],
                "Delete",
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenterByRight { offset: 290.0 },
                    y: Vertical::AtWindowCenter { offset: y + 25.0 },
                },
            )));
            y += 50.0;
        }
        LoadWorld { sprites }
    }
}

impl Scene for LoadWorld {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "back" => Some(Transition::Pop),
            _ => {
                if btn_id.starts_with("del:") {
                    let path = btn_id.strip_prefix("del:").unwrap();
                    delete(path.parse().unwrap());
                    Some(Transition::Pop)
                } else {
                    println!("{}", btn_id);
                    None
                }
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Ok(Transition::Pop)
        } else if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Box<dyn Sprite>>> {
        Some(&mut self.sprites)
    }
}
