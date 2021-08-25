use assets::Assets;
use colors::Colors;
use scene_manager::{Scene, Transition};
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::MouseButton;
use tetra::{input, Context};

pub struct SettingsScene {
    sprites: Vec<Box<dyn Sprite>>,
}

impl SettingsScene {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let title = Label::new(
            "Settings",
            assets.borrow().avqest.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(30.0, AnchorY::Top),
        );
        Ok(SettingsScene {
            sprites: vec![Box::new(bg), Box::new(title)],
        })
    }
}

impl Scene for SettingsScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn sprites(&mut self) -> &mut Vec<Box<dyn Sprite>> {
        &mut self.sprites
    }
}
