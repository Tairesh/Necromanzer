use app::App;
use assets::names::Names;
use colors::Colors;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use savefile;
use scenes::scene::Scene;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{bg, easy_back, title};
use sprites::button::Button;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::{Draw, Positionate, Stringify};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

const RANDOMIZE_EVENT: &str = "r";
const CREATE_EVENT: &str = "c";

fn random_seed<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.next_u32().to_string()
}

pub struct CreateWorld {
    names: Names,
    sprites: BunchOfSprites,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    name_error: Rc<RefCell<Label>>,
    seed_input: Rc<RefCell<TextInput>>,
    seed_error: Rc<RefCell<Label>>,
}

impl CreateWorld {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let mut rng = thread_rng();

        let bg = bg(&app.assets, &app.settings);
        let title = title("Create new world:", &app.assets);

        let name_label = Rc::new(RefCell::new(Label::new(
            "World name:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::ByCenter { y: 195.0 },
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            *app.assets.names.names.choose(&mut rng).unwrap(),
            250.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
        )));
        let name_error = Rc::new(RefCell::new(Label::hidden(
            "Savefile with this name already exists",
            app.assets.fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                y: Vertical::ByBottom { y: 180.0 },
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "World name shall not be empty!",
            app.assets.fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                y: Vertical::ByBottom { y: 180.0 },
            },
        )));
        let seed_label = Rc::new(RefCell::new(Label::new(
            "World seed:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: Vertical::ByCenter { y: 265.0 },
            },
        )));
        let seed_input = Rc::new(RefCell::new(TextInput::new(
            random_seed(&mut rng).as_str(),
            250.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: Vertical::ByCenter { y: 270.0 },
            },
        )));
        let seed_error = Rc::new(RefCell::new(Label::hidden(
            "Seed shall not be empty!",
            app.assets.fonts.default.clone(),
            Colors::DARK_RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 125.0 },
                y: Vertical::ByBottom { y: 250.0 },
            },
        )));
        let randomize_btn = Rc::new(RefCell::new(Button::text(
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(RANDOMIZE_EVENT.to_string()),
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0 - 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::Pop,
        )));
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(CREATE_EVENT.to_string()),
        )));

        Self {
            names: app.assets.names.clone(),
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

impl Scene for CreateWorld {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        let focused = self.is_there_focused_sprite();
        easy_back(event, focused)
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: &str) -> SomeTransitions {
        match event {
            RANDOMIZE_EVENT => {
                let mut rng = rand::thread_rng();
                self.name_input
                    .borrow_mut()
                    .set_value(*self.names.names.choose(&mut rng).unwrap());
                self.seed_input
                    .borrow_mut()
                    .set_value(random_seed(&mut rng).as_str());
                None
            }
            CREATE_EVENT => {
                let seed = self.seed_input.borrow().value();
                let name = self.name_input.borrow().value();
                if seed.is_empty() {
                    self.seed_input.borrow_mut().set_danger(true);
                    self.seed_error.borrow_mut().set_visible(true);
                }
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                    None
                } else {
                    match savefile::create(name.as_str(), seed.as_str()) {
                        Ok(_) => Some(vec![Transition::Pop]),
                        Err(err) => match err {
                            savefile::SaveError::SystemError(err) => {
                                panic!("Can't write savefile: {}", err)
                            }
                            savefile::SaveError::SerializeError(err) => {
                                panic!("Can't create savefile: {}", err)
                            }
                            savefile::SaveError::FileExists => {
                                self.name_input.borrow_mut().set_danger(true);
                                self.name_error.borrow_mut().set_visible(true);
                                None
                            }
                        },
                    }
                }
            }
            _ => None,
        }
    }
}
