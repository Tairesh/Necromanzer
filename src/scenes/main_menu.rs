use assets::Assets;
use colors::Colors;
use savefile::savefiles;
use scenes::create_world::CreateWorld;
use scenes::load_world::LoadWorld;
use scenes::manager::{update_sprites, Scene, Transition};
use scenes::settings::SettingsScene;
use settings::Settings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position, Vertical};
use sprites::sprite::{Disable, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;
use VERSION;

pub struct MainMenu {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    select_btn: Rc<RefCell<Button>>,
}

impl MainMenu {
    pub fn new(assets: Rc<RefCell<Assets>>, settings: Rc<RefCell<Settings>>) -> Self {
        let bg = Rc::new(RefCell::new(Image::new(
            assets.borrow().bg.clone(),
            Position::center(),
        )));
        let logo = Rc::new(RefCell::new(Image::new(
            assets.borrow().logo.clone(),
            Position::horizontal_center(0.0, 50.0, AnchorY::Top),
        )));
        let version = Rc::new(RefCell::new(Label::new(
            &*VERSION,
            assets.borrow().default.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, 69.69, AnchorY::Top),
        )));
        let select_btn = Rc::new(RefCell::new(
            Button::new(
                "select_world",
                vec![(Key::E, None)],
                "[e] Select world",
                assets.clone(),
                Position {
                    x: Horizontal::AtWindowCenter { offset: 0.0 },
                    y: Vertical::AtWindowCenter { offset: 0.0 },
                },
            )
            .with_disabled(true),
        ));
        let create_btn = Rc::new(RefCell::new(Button::new(
            "create_world",
            vec![(Key::C, None)],
            "[c] Create new world",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 50.0 },
            },
        )));
        let settings_btn = Rc::new(RefCell::new(Button::new(
            "settings",
            vec![(Key::S, None)],
            "[s] Settings",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 100.0 },
            },
        )));
        let exit_btn = Rc::new(RefCell::new(Button::new(
            "exit",
            vec![(Key::X, None)],
            "[x] Exit",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowCenter { offset: 150.0 },
            },
        )));
        MainMenu {
            assets,
            settings,
            sprites: vec![
                bg,
                logo,
                version,
                select_btn.clone(),
                create_btn,
                settings_btn,
                exit_btn,
            ],
            select_btn,
        }
    }
}

impl Scene for MainMenu {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "exit" => Some(Transition::Quit),
            "settings" => Some(Transition::Push(Box::new(SettingsScene::new(
                self.assets.clone(),
                self.settings.clone(),
                ctx,
            )))),
            "create_world" => Some(Transition::Push(Box::new(CreateWorld::new(
                self.assets.clone(),
                ctx,
            )))),
            "select_world" => Some(Transition::Push(Box::new(LoadWorld::new(
                self.assets.clone(),
                ctx,
            )))),
            _ => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        update_sprites(self, ctx)
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }

    fn on_open(&mut self, ctx: &mut Context) {
        self.select_btn
            .borrow_mut()
            .set_disabled(savefiles().is_empty());
        self.on_resize(ctx);
    }
}
