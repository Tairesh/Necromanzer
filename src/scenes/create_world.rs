use assets::Assets;
use colors::Colors;
use rand::seq::SliceRandom;
use rand::RngCore;
use savefile::{SaveFile, SaveFileError};
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::{Draw, Positionate, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

#[allow(dead_code)]
pub struct CreateWorld {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Box<dyn Sprite>>,
}

impl CreateWorld {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        ctx: &mut Context,
    ) -> Self {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let title = Label::new(
            "Create new world:",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        );
        let name_label = Label::new(
            "World name:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(195.0),
            },
        );
        let name_input = TextInput::new(
            "world_name",
            *assets
                .borrow()
                .names
                .choose(&mut rand::thread_rng())
                .unwrap(),
            250.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        );
        let mut name_error = Label::new(
            "Savefile with this name already exists",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Bottom.to_position(180.0),
            },
        );
        name_error.set_visible(false);
        let seed_label = Label::new(
            "World seed:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(255.0),
            },
        );
        let seed_input = TextInput::new(
            "world_seed",
            random_seed().as_str(),
            250.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(260.0),
            },
        );
        let mut seed_error = Label::new(
            "Seed can't be empty",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Bottom.to_position(240.0),
            },
        );
        seed_error.set_visible(false);
        let mut randomize_btn = Button::new(
            "randomize",
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        let randomize_size = randomize_btn.calc_size(ctx);
        let back_btn = Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        let create_btn = Button::new(
            "create",
            vec![(Key::Enter, None)],
            "[Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        CreateWorld {
            assets,
            settings,
            sprites: vec![
                Box::new(bg),
                Box::new(title),
                Box::new(name_label),
                Box::new(name_input),
                Box::new(seed_label),
                Box::new(seed_input),
                Box::new(back_btn),
                Box::new(randomize_btn),
                Box::new(create_btn),
                Box::new(name_error),
                Box::new(seed_error),
            ],
        }
    }
}

fn random_seed() -> String {
    rand::thread_rng().next_u32().to_string()
}

impl Scene for CreateWorld {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "randomize" => {
                self.sprites.get_mut(3).unwrap().set_value(
                    *self
                        .assets
                        .borrow()
                        .names
                        .choose(&mut rand::thread_rng())
                        .unwrap(),
                );
                self.sprites
                    .get_mut(5)
                    .unwrap()
                    .set_value(random_seed().as_str());
            }
            "create" => {
                if self.sprites.get(5).unwrap().get_value().unwrap().is_empty() {
                    self.sprites.get_mut(5).unwrap().set_danger(true);
                    self.sprites.get_mut(10).unwrap().set_visible(true);
                } else {
                    let file = SaveFile::new(
                        self.sprites.get(3).unwrap().get_value().unwrap().as_str(),
                        self.sprites.get(5).unwrap().get_value().unwrap().as_str(),
                    );
                    match file.save() {
                        Ok(_) => return Some(Transition::Pop),
                        Err(err) => match err {
                            SaveFileError::SystemError(err) => {
                                panic!("Can't create savefile: {}", err)
                            }
                            SaveFileError::FileExists => {
                                self.sprites.get_mut(3).unwrap().set_danger(true);
                                self.sprites.get_mut(9).unwrap().set_visible(true);
                            }
                        },
                    }
                }
            }
            "back" => return Some(Transition::Pop),
            _ => {}
        }
        None
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if !self.sprites.get(3).unwrap().get_danger() && self.sprites.get(9).unwrap().visible() {
            self.sprites.get_mut(9).unwrap().set_visible(false);
        }
        if !self.sprites.get(5).unwrap().get_danger() && self.sprites.get(10).unwrap().visible() {
            self.sprites.get_mut(10).unwrap().set_visible(false);
        }
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