use assets::Assets;
use colors::Colors;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::{input, Context};

#[allow(dead_code)]
pub struct CreateWorld {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Box<dyn Sprite>>,
}

impl CreateWorld {
    pub fn new(assets: Rc<RefCell<Assets>>, settings: Rc<RefCell<Settings>>) -> Self {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let title = Label::new(
            "Create new world:",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        );
        let name_label = Label::new(
            "World name:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(195.0),
            },
        );
        let name_input = TextInput::new(
            "world_name",
            "Tadek",
            250.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -0.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        );
        let seed_label = Label::new(
            "World seed:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(245.0),
            },
        );
        let seed_input = TextInput::new(
            "world_seed",
            "1233254234523",
            250.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -0.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        );
        let randomize_btn = Button::new(
            "randomize",
            Some(Key::NumPadMultiply),
            "[*] Randomize",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        let create_btn = Button::new(
            "create",
            Some(Key::Enter),
            "[Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        CreateWorld {
            assets,
            settings,
            sprites: vec![
                Box::new(bg),
                Box::new(title),
                Box::new(name_label),
                Box::new(name_input),
                Box::new(seed_label),
                Box::new(seed_input),
                Box::new(randomize_btn),
                Box::new(create_btn),
            ],
        }
    }
}

impl Scene for CreateWorld {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Ok(Transition::Pop)
        } else if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Box<dyn Sprite>>> {
        Some(&mut self.sprites)
    }
}
