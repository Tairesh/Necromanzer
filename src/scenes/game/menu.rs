use assets::Assets;
use scenes::manager::{update_sprites, Scene, Transition};
use scenes::settings::SettingsScene;
use settings::Settings;
use sprites::alert::Alert;
use sprites::button::Button;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::{input, Context};

pub struct Menu {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl Menu {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        _ctx: &mut Context,
    ) -> Self {
        let alert = Rc::new(RefCell::new(Alert::new(
            200.0,
            190.0,
            assets.borrow().alert.clone(),
            assets.borrow().alert_nineslice.clone(),
            Position::center(),
        )));
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: -30.0 },
            },
        )));
        let settings_btn = Rc::new(RefCell::new(Button::new(
            "settings",
            vec![(Key::S, None)],
            "[S] Settings",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 20.0 },
            },
        )));
        let quit_btn = Rc::new(RefCell::new(Button::new(
            "quit",
            vec![(Key::Q, None)],
            "[q] Quit",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenterByBottom { offset: 70.0 },
            },
        )));
        Self {
            assets,
            settings,
            sprites: vec![alert, back_btn, settings_btn, quit_btn],
        }
    }
}

impl Scene for Menu {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "back" => Some(Transition::Pop),
            "settings" => Some(Transition::Replace(Box::new(SettingsScene::new(
                self.assets.clone(),
                self.settings.clone(),
                ctx,
            )))),
            "quit" => Some(Transition::Pop2),
            _ => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
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
