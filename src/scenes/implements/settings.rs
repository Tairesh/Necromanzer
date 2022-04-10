use app::App;
use colors::Colors;
use scenes::scene::Scene;
use scenes::transition::{SettingsChange, SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, title};
use sprites::button::Button;
use sprites::label::Label;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::{Positionate, Press};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

const WINDOW_MODE_EVENT: u8 = 1;
const FULLSCREEN_MODE_EVENT: u8 = 2;

pub struct Settings {
    sprites: BunchOfSprites,
    window_btn: Rc<RefCell<Button>>,
    fullscreen_btn: Rc<RefCell<Button>>,
}

impl Settings {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let assets = &app.assets;
        let settings = &app.settings;
        let bg = bg(assets, settings);
        let title = title("Settings", &app.assets);

        let fullscreen_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::F, Some(KeyModifier::Alt))],
            "[Alt+F] Fullscreen",
            assets.fonts.default.clone(),
            assets.button.clone(),
            settings.window_settings.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 100.0 },
                y: Vertical::ByCenter { y: 150.0 },
            },
            Transition::CustomEvent(FULLSCREEN_MODE_EVENT),
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            assets.fonts.default.clone(),
            assets.button.clone(),
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
            assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_btn_size.x,
                },
                y: Vertical::ByCenter { y: 145.0 },
            },
        )));
        let back_btn = back_btn(
            Position::horizontal_center(0.0, Vertical::AtWindowBottomByBottom { offset: -200.0 }),
            assets,
        );

        Self {
            sprites: vec![
                bg,
                title,
                fullscreen_btn.clone(),
                window_btn.clone(),
                window_mode_label,
                back_btn,
            ],
            fullscreen_btn,
            window_btn,
        }
    }
}

impl Scene for Settings {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(event, self.is_there_focused_sprite())
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, _ctx: &mut Context, event: u8) -> SomeTransitions {
        match event {
            FULLSCREEN_MODE_EVENT => {
                self.window_btn.borrow_mut().unpress();
                // TODO: this will be not needed if buttons will return SomeTransitions on click, not one
                Some(vec![Transition::ChangeSettings(
                    SettingsChange::FullscreenMode,
                )])
            }
            WINDOW_MODE_EVENT => {
                self.fullscreen_btn.borrow_mut().unpress();
                Some(vec![Transition::ChangeSettings(SettingsChange::WindowMode)])
            }
            _ => None,
        }
    }
}
