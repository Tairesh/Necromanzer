use assets::Assets;
use colors::Colors;
use rand::seq::SliceRandom;
use rand::RngCore;
use savefile::{CreateFileError, SaveFile};
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::{Draw, Positionate, Sprite, Stringify};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

pub struct CreateWorld {
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    name_error: Rc<RefCell<Label>>,
    seed_input: Rc<RefCell<TextInput>>,
    seed_error: Rc<RefCell<Label>>,
}

impl CreateWorld {
    pub fn new(assets: Rc<RefCell<Assets>>, ctx: &mut Context) -> Self {
        let bg = Rc::new(RefCell::new(Image::new(
            assets.borrow().images.bg.clone(),
            Position::center(),
        )));
        let title = Rc::new(RefCell::new(Label::new(
            "Create new world:",
            assets.borrow().fonts.header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        )));
        let name_label = Rc::new(RefCell::new(Label::new(
            "World name:",
            assets.borrow().fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(195.0),
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            *assets
                .borrow()
                .names
                .names
                .choose(&mut rand::thread_rng())
                .unwrap(),
            250.0,
            assets.borrow().fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        )));
        let name_error = Rc::new(RefCell::new(Label::hidden(
            "Savefile with this name already exists",
            assets.borrow().fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: AnchorY::Bottom.to_position(170.0),
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "World name shall not be empty!",
            assets.borrow().fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: AnchorY::Bottom.to_position(170.0),
            },
        )));
        let seed_label = Rc::new(RefCell::new(Label::new(
            "World seed:",
            assets.borrow().fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(265.0),
            },
        )));
        let seed_input = Rc::new(RefCell::new(TextInput::new(
            random_seed().as_str(),
            250.0,
            assets.borrow().fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(270.0),
            },
        )));
        let seed_error = Rc::new(RefCell::new(Label::hidden(
            "Seed shall not be empty!",
            assets.borrow().fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 125.0 },
                y: AnchorY::Bottom.to_position(240.0),
            },
        )));
        let randomize_btn = Rc::new(RefCell::new(Button::new(
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
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0 - 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        )));
        let create_btn = Rc::new(RefCell::new(Button::new(
            "create",
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        )));
        CreateWorld {
            assets,
            sprites: vec![
                bg,
                title,
                name_label,
                name_input.clone(),
                seed_label,
                seed_input.clone(),
                back_btn,
                randomize_btn,
                create_btn,
                name_error.clone(),
                seed_error.clone(),
                name_empty.clone(),
            ],
            name_input,
            name_error,
            name_empty,
            seed_input,
            seed_error,
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
                self.name_input.borrow_mut().set_value(
                    *self
                        .assets
                        .borrow()
                        .names
                        .names
                        .choose(&mut rand::thread_rng())
                        .unwrap(),
                );
                self.seed_input
                    .borrow_mut()
                    .set_value(random_seed().as_str());
            }
            "create" => {
                let seed = self.seed_input.borrow().value();
                let name = self.name_input.borrow().value();
                if seed.is_empty() {
                    self.seed_input.borrow_mut().set_danger(true);
                    self.seed_error.borrow_mut().set_visible(true);
                }
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                } else {
                    let mut file = SaveFile::new(name.as_str(), seed.as_str());
                    match file.create() {
                        Ok(_) => return Some(Transition::Pop),
                        Err(err) => match err {
                            CreateFileError::SystemError(err) => {
                                panic!("Can't create savefile: {}", err)
                            }
                            CreateFileError::FileExists => {
                                self.name_input.borrow_mut().set_danger(true);
                                self.name_error.borrow_mut().set_visible(true);
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

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        {
            let name = self.name_input.borrow();
            let mut name_empty = self.name_empty.borrow_mut();
            let mut name_error = self.name_error.borrow_mut();
            let seed = self.seed_input.borrow();
            let mut seed_error = self.seed_error.borrow_mut();
            if !name.danger() && name_empty.visible() {
                name_empty.set_visible(false);
            }
            if !name.danger() && name_error.visible() {
                name_error.set_visible(false);
            }
            if !seed.danger() && seed_error.visible() {
                seed_error.set_visible(false);
            }
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
