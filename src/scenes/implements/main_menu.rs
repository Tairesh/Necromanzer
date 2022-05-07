use app::App;
use colors::Colors;
use savefile::savefiles_exists;
use scenes::bg;
use scenes::scene::Scene;
use scenes::scene_impl::SceneImpl;
use scenes::transition::Transition;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::Context;
use ui::button::Button;
use ui::image::Image;
use ui::label::Label;
use ui::position::{Position, Vertical};
use ui::traits::Disable;
use ui::{BunchOfSprites, SomeSprites};
use VERSION;

pub struct MainMenu {
    sprites: BunchOfSprites,
    select_btn: Rc<RefCell<Button>>,
}

impl MainMenu {
    pub fn new(app: &App) -> Self {
        let bg = bg(&app.assets, &app.settings.borrow());
        let logo = Rc::new(RefCell::new(Image::new(
            app.assets.images.logo.clone(),
            Position::horizontal_center(0.0, Vertical::ByTop { y: 50.0 }),
        )));
        let version = Rc::new(RefCell::new(Label::new(
            VERSION,
            app.assets.fonts.default.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 69.69 }),
        )));
        let select_btn = Rc::new(RefCell::new(
            Button::text(
                vec![Key::E.into()],
                "[e] Select world",
                app.assets.fonts.default.clone(),
                app.assets.button.clone(),
                Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 0.0 }),
                Transition::Push(Scene::LoadWorld),
            )
            .with_disabled(true),
        ));
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![Key::C.into()],
            "[c] Create new world",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 50.0 }),
            Transition::Push(Scene::CreateWorld),
        )));
        let settings_btn = Rc::new(RefCell::new(Button::text(
            vec![Key::S.into()],
            "[s] Settings",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByTop { offset: 100.0 }),
            Transition::Push(Scene::Settings),
        )));
        let exit_btn = Rc::new(RefCell::new(Button::text(
            vec![Key::X.into()],
            "[x] Exit",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
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

impl SceneImpl for MainMenu {
    fn on_open(&mut self, _ctx: &mut Context) {
        self.select_btn
            .borrow_mut()
            .set_disabled(!savefiles_exists());
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }
}
