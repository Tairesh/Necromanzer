use assets::Assets;
use colors::Colors;
use scenes::scene::Scene;
use scenes::transition::{SettingsChange, SomeTransitions, Transition};
use scenes::{bg, easy_back};
use settings::game::GameSettings;
use sprites::button::Button;
use sprites::label::Label;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::{Positionate, Press};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};

const WINDOW_MODE_EVENT: &str = "w";
const FULLSCREEN_MODE_EVENT: &str = "f";

pub struct Settings {
    sprites: BunchOfSprites,
    window_btn: Rc<RefCell<Button>>,
    fullscreen_btn: Rc<RefCell<Button>>,
}

impl Settings {
    pub fn new(ctx: &mut Context, assets: &Assets, settings: &GameSettings) -> Self {
        let bg = bg(assets, settings);
        let title = Rc::new(RefCell::new(Label::new(
            "Settings",
            assets.fonts.header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 20.0 }),
        )));

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
            Transition::CustomEvent(FULLSCREEN_MODE_EVENT.to_string()),
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
            Transition::CustomEvent(WINDOW_MODE_EVENT.to_string()),
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
                y: Vertical::ByCenter { y: 150.0 },
            },
        )));
        let back_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.fonts.default.clone(),
            assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowBottomByBottom { offset: -200.0 }),
            Transition::Pop,
        )));

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

    fn custom_event(&mut self, _ctx: &mut Context, event: &str) -> SomeTransitions {
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
