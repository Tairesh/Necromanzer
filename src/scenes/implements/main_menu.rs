use assets::Assets;
use colors::Colors;
use scenes::bg;
use scenes::scene::Scene;
use scenes::transition::Transition;
use settings::game::GameSettings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;
use VERSION;

pub struct MainMenu {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    select_btn: Rc<RefCell<Button>>,
}

impl MainMenu {
    pub fn new(_ctx: &mut Context, assets: &Assets, settings: &GameSettings) -> Self {
        let bg = bg(assets, settings);
        let logo = Rc::new(RefCell::new(Image::new(
            assets.images.logo.clone(),
            Position::horizontal_center(0.0, Vertical::ByTop { y: 50.0 }),
        )));
        let version = Rc::new(RefCell::new(Label::new(
            VERSION,
            assets.fonts.default.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 69.69 }),
        )));
        let select_btn = Rc::new(RefCell::new(
            Button::text(
                vec![(Key::E, None)],
                "[e] Select world",
                assets.fonts.default.clone(),
                assets.button.clone(),
                Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 0.0 }),
                Transition::Quit,
            )
            .with_disabled(true),
        ));
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::C, None)],
            "[c] Create new world",
            assets.fonts.default.clone(),
            assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 50.0 }),
            Transition::Quit,
        )));
        let settings_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::S, None)],
            "[s] Settings",
            assets.fonts.default.clone(),
            assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 100.0 }),
            Transition::Quit,
        )));
        let exit_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::X, None)],
            "[x] Exit",
            assets.fonts.default.clone(),
            assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 150.0 }),
            Transition::Quit,
        )));

        Self {
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
    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
