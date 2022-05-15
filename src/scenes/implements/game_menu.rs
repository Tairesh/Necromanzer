use tetra::input::Key;

use app::App;
use scenes::scene::Scene;
use scenes::scene_impl::SceneImpl;
use scenes::transition::Transition;
use ui::alert::Alert;
use ui::button::Button;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::UiSprite;
use ui::{SomeUISprites, SomeUISpritesMut};

pub struct GameMenu {
    sprites: [Box<dyn UiSprite>; 4],
}

impl GameMenu {
    pub fn new(app: &App) -> Self {
        let alert = Box::new(Alert::new(
            200.0,
            190.0,
            app.assets.alert.clone(),
            Position::center(),
        ));
        let back_btn = Box::new(Button::text(
            vec![Key::Escape.into()],
            "[Esc] Back",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -30.0 },
            },
            Transition::Pop,
        ));
        let settings_btn = Box::new(Button::text(
            vec![Key::S.into()],
            "[S] Settings",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 20.0 },
            },
            Transition::Replace(Scene::Settings),
        ));
        let quit_btn = Box::new(Button::text(
            vec![Key::Q.into()],
            "[q] Quit",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 70.0 },
            },
            Transition::GoMainMenu,
        ));

        Self {
            sprites: [alert, back_btn, settings_btn, quit_btn],
        }
    }
}

impl SceneImpl for GameMenu {
    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }
}
