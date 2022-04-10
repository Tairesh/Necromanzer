use app::App;
use scenes::game_scene::GameScene;
use scenes::scene::Scene;
use scenes::transition::Transition;
use sprites::alert::Alert;
use sprites::button::Button;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;

pub struct GameMenu {
    sprites: BunchOfSprites,
}

impl GameMenu {
    pub fn new(app: &App) -> Self {
        let alert = Rc::new(RefCell::new(Alert::new(
            200.0,
            190.0,
            app.assets.alert.clone(),
            Position::center(),
        )));
        let back_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Escape, None)],
            "[Esc] Back",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -30.0 },
            },
            Transition::Pop,
        )));
        let settings_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::S, None)],
            "[S] Settings",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 20.0 },
            },
            Transition::Replace(GameScene::Settings),
        )));
        let quit_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Q, None)],
            "[q] Quit",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 70.0 },
            },
            Transition::GoMainMenu,
        )));

        Self {
            sprites: vec![alert, back_btn, settings_btn, quit_btn],
        }
    }
}

impl Scene for GameMenu {
    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }
}
