use assets::Assets;
use colors::Colors;
use scene_manager::{Scene, Transition};
use scenes::settings::SettingsScene;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::math::Vec2;
use tetra::{input, Context};
use VERSION;

pub struct MainMenu {
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Box<dyn Sprite>>,
}

impl MainMenu {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let logo = Image::new(
            assets.borrow().logo.clone(),
            Position::horizontal_center(50.0, AnchorY::Top),
        );
        let version = Label::new(
            &*VERSION,
            assets.borrow().consolab.clone(),
            Colors::DARK_GRAY,
            Position::empty(),
        );
        Ok(MainMenu {
            assets,
            sprites: vec![Box::new(bg), Box::new(logo), Box::new(version)],
        })
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            let scene = SettingsScene::new(Rc::clone(&self.assets))?;
            Ok(Transition::Push(Box::new(scene)))
        } else if input::is_key_released(ctx, Key::X) {
            Ok(Transition::Quit)
        } else {
            Ok(Transition::None)
        }
    }

    fn on_resize(&mut self, ctx: &mut Context) -> tetra::Result {
        let logo = self.sprites.get_mut(1).unwrap();
        let logo_vec: Vec2<f32> = logo.calc_position(ctx);
        let logo_size = logo.size(ctx);
        let version = self.sprites.get_mut(2).unwrap();
        version.set_position(Position::new(
            logo_vec.x + logo_size.0 - 22.0,
            logo_vec.y + 17.0,
            AnchorX::Right,
            AnchorY::Top,
        ));

        for sprite in self.sprites.iter_mut() {
            sprite.calc_position(ctx);
        }
        self.clear(ctx)
    }

    fn sprites(&mut self) -> &mut Vec<Box<dyn Sprite>> {
        &mut self.sprites
    }
}
