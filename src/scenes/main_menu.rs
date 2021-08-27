use assets::Assets;
use colors::Colors;
use scenes::manager::{update_sprites, Scene, Transition};
use scenes::settings::SettingsScene;
use settings::Settings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;
use VERSION;

pub struct MainMenu {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Box<dyn Sprite>>,
}

impl MainMenu {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
    ) -> tetra::Result<Self> {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let logo = Image::new(
            assets.borrow().logo.clone(),
            Position::horizontal_center(0.0, 50.0, AnchorY::Top),
        );
        let version = Label::new(
            &*VERSION,
            assets.borrow().default.clone(),
            Colors::LIGHT_YELLOW,
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowBottom { offset: -10.0 },
            },
        );
        let select_btn = Button::new(
            "select_world",
            Some(Key::L),
            "[e] Select world",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 0.0 },
            },
        )
        .with_disabled(true);
        let create_btn = Button::new(
            "create_world",
            Some(Key::C),
            "[c] Create new world",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 50.0 },
            },
        );
        let settings_btn = Button::new(
            "settings",
            Some(Key::S),
            "[s] Settings",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 100.0 },
            },
        );
        let exit_btn = Button::new(
            "exit",
            Some(Key::X),
            "[x] Exit",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 150.0 },
            },
        );
        Ok(MainMenu {
            assets,
            settings,
            sprites: vec![
                Box::new(bg),
                Box::new(logo),
                Box::new(version),
                Box::new(select_btn),
                Box::new(create_btn),
                Box::new(settings_btn),
                Box::new(exit_btn),
            ],
        })
    }
}

impl Scene for MainMenu {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "exit" => Some(Transition::Quit),
            "settings" => Some(Transition::Push(Box::new(
                SettingsScene::new(self.assets.clone(), self.settings.clone(), ctx).unwrap(),
            ))),
            _ => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn sprites(&mut self) -> &mut Vec<Box<dyn Sprite>> {
        &mut self.sprites
    }
}
