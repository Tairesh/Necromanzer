use std::cell::RefCell;
use std::rc::Rc;

use rand::{thread_rng, Rng};
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

use app::App;
use colors::Colors;
use game::GameData;
use savefile;
use scenes::scene::Scene;
use scenes::scene_impl::SceneImpl;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, title};
use ui::button::Button;
use ui::inputs::TextInput;
use ui::label::Label;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::{Draw, Positionate, Stringify};
use ui::{BunchOfSprites, SomeSprites};

const RANDOMIZE_EVENT: u8 = 1;
const CREATE_EVENT: u8 = 2;

fn random_seed<R: Rng + ?Sized>(rng: &mut R) -> String {
    rng.next_u32().to_string()
}

pub struct CreateWorld {
    sprites: BunchOfSprites,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    name_error: Rc<RefCell<Label>>,
    seed_input: Rc<RefCell<TextInput>>,
    seed_error: Rc<RefCell<Label>>,
}

impl CreateWorld {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let mut rng = thread_rng();

        let bg = bg(&app.assets);
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
            GameData::instance().names.random_name(&mut rng),
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
                Key::NumPadMultiply.into(),
                (Key::Num8, KeyModifier::Shift).into(),
            ],
            "[*] Randomize",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(RANDOMIZE_EVENT),
        )));
        let randomize_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0 - 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            &app.assets,
        );
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Enter, KeyModifier::Alt).into()],
            "[Alt+Enter] Create",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(CREATE_EVENT),
        )));

        Self {
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

impl SceneImpl for CreateWorld {
    fn on_update(&mut self, _ctx: &mut Context) -> SomeTransitions {
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
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        let focused = self.is_there_focused_sprite();
        easy_back(&event, focused)
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> SomeTransitions {
        match event {
            RANDOMIZE_EVENT => {
                let mut rng = rand::thread_rng();
                self.name_input
                    .borrow_mut()
                    .set_value(GameData::instance().names.random_name(&mut rng));
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
                        Ok(path) => Some(vec![Transition::Replace(Scene::CreateCharacter(path))]),
                        Err(err) => match err {
                            savefile::Error::System(err) => {
                                panic!("Can't write savefile: {}", err)
                            }
                            savefile::Error::Serialize(err) => {
                                panic!("Can't create savefile: {}", err)
                            }
                            savefile::Error::FileExists => {
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
