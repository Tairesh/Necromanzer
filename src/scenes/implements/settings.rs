use std::cell::RefCell;
use std::rc::Rc;

use tetra::input::{Key, KeyModifier};
use tetra::window::WindowPosition;
use tetra::{Context, Event};

use app::App;
use colors::Colors;
use scenes::scene_impl::SceneImpl;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, title};
use settings::game::GameSettings;
use ui::button::Button;
use ui::input::TextInput;
use ui::label::Label;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::{Positionate, Press, Stringify};
use ui::{BunchOfSprites, SomeSprites};

const WINDOW_MODE_EVENT: u8 = 1;
const FULLSCREEN_MODE_EVENT: u8 = 2;
const REPEAT_INTERVAL_MINUS: u8 = 3;
const REPEAT_INTERVAL_PLUS: u8 = 4;

pub struct Settings {
    sprites: BunchOfSprites,
    window_btn: Rc<RefCell<Button>>,
    fullscreen_btn: Rc<RefCell<Button>>,
    repeat_interval_input: Rc<RefCell<TextInput>>,
    settings: Rc<RefCell<GameSettings>>,
}

impl Settings {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let settings = app.settings.borrow();
        let bg = bg(&app.assets, &settings);
        let title = title("Settings", &app.assets);

        let fullscreen_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::F, KeyModifier::Alt).into()],
            "[Alt+F] Fullscreen",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            settings.window_settings.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 100.0 },
                y: Vertical::ByCenter { y: 150.0 },
            },
            Transition::CustomEvent(FULLSCREEN_MODE_EVENT),
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::W, KeyModifier::Alt).into()],
            "[Alt+W] Window",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            !settings.window_settings.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 98.0 },
                y: Vertical::ByCenter { y: 150.0 },
            },
            Transition::CustomEvent(WINDOW_MODE_EVENT),
        )));
        let window_btn_size = window_btn.borrow_mut().calc_size(ctx);

        let window_mode_label = Rc::new(RefCell::new(Label::new(
            "Window mode:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_btn_size.x,
                },
                y: Vertical::ByCenter { y: 145.0 },
            },
        )));

        let repeat_interval_label = Rc::new(RefCell::new(Label::new(
            "Repeat delay:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_btn_size.x,
                },
                y: Vertical::ByCenter { y: 195.0 },
            },
        )));
        let repeat_interval_minus = Rc::new(RefCell::new(Button::icon(
            vec![],
            "minus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 0.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
            Transition::CustomEvent(REPEAT_INTERVAL_MINUS),
        )));
        let repeat_interval_input = Rc::new(RefCell::new(TextInput::int(
            app.settings.borrow().repeat_interval as u32,
            (1, 10000),
            190.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
        )));
        let repeat_interval_plus = Rc::new(RefCell::new(Button::icon(
            vec![],
            "plus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 200.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
            Transition::CustomEvent(REPEAT_INTERVAL_PLUS),
        )));

        let back_btn = back_btn(
            Position::horizontal_center(0.0, Vertical::AtWindowBottomByBottom { offset: -200.0 }),
            &app.assets,
        );

        Self {
            sprites: vec![
                bg,
                title,
                fullscreen_btn.clone(),
                window_btn.clone(),
                window_mode_label,
                repeat_interval_label,
                repeat_interval_minus,
                repeat_interval_input.clone(),
                repeat_interval_plus,
                back_btn,
            ],
            settings: app.settings.clone(),
            fullscreen_btn,
            window_btn,
            repeat_interval_input,
        }
    }
}

impl SceneImpl for Settings {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(event, self.is_there_focused_sprite())
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        match event {
            FULLSCREEN_MODE_EVENT => {
                self.window_btn.borrow_mut().unpress();
                if !tetra::window::is_fullscreen(ctx) {
                    self.settings.borrow_mut().window_settings.fullscreen = true;
                    tetra::window::set_fullscreen(ctx, true).ok();
                }
                None
            }
            WINDOW_MODE_EVENT => {
                self.fullscreen_btn.borrow_mut().unpress();
                if tetra::window::is_fullscreen(ctx) {
                    self.settings.borrow_mut().window_settings.fullscreen = false;
                    tetra::window::set_fullscreen(ctx, false).ok();
                    tetra::window::set_decorated(ctx, true);
                    tetra::window::set_size(
                        ctx,
                        self.settings.borrow().window_settings.width as i32,
                        self.settings.borrow().window_settings.height as i32,
                    )
                    .ok();
                    let current_monitor = tetra::window::get_current_monitor(ctx).unwrap_or(0);
                    tetra::window::set_position(
                        ctx,
                        WindowPosition::Centered(current_monitor),
                        WindowPosition::Centered(current_monitor),
                    );
                }
                None
            }
            REPEAT_INTERVAL_MINUS | REPEAT_INTERVAL_PLUS => {
                let mut input = self.repeat_interval_input.borrow_mut();
                if let Ok(mut value) = input.value().parse::<u8>() {
                    match event {
                        REPEAT_INTERVAL_MINUS => {
                            value -= 1;
                        }
                        REPEAT_INTERVAL_PLUS => {
                            value += 1;
                        }
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{}", value).as_str());
                }
                None
            }
            _ => None,
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        if let Ok(repeat_interval) = self.repeat_interval_input.borrow().value().parse() {
            self.settings.borrow_mut().repeat_interval = repeat_interval;
            self.settings.borrow_mut().save();
        }
    }
}
