use assets::Assets;
use colors::Colors;
use savefile::SaveFile;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::{Positionate, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

#[allow(dead_code)]
pub struct CreateCharacter {
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Box<dyn Sprite>>,
    savefile: SaveFile,
}

impl CreateCharacter {
    pub fn new(assets: Rc<RefCell<Assets>>, savefile: SaveFile, ctx: &mut Context) -> Self {
        let mut sprites: Vec<Box<dyn Sprite>> = Vec::new();
        sprites.push(Box::new(Image::new(
            assets.borrow().bg.clone(),
            Position::center(),
        )));
        sprites.push(Box::new(Label::new(
            "Create new character:",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        )));
        sprites.push(Box::new(Label::new(
            format!("New adventurer in the «{}» world", savefile.name).as_str(),
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, 100.0, AnchorY::Top),
        )));
        sprites.push(Box::new(Label::new(
            "Name:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(195.0),
            },
        )));
        sprites.push(Box::new(TextInput::new(
            "name",
            "",
            250.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        )));
        sprites.push(Box::new(Label::hidden(
            "Character with this name already exists",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Bottom.to_position(180.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Gender:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(245.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Age:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(295.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Main hand:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(345.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Skin tone:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(395.0),
            },
        )));
        let mut randomize_btn = Button::new(
            "randomize",
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        let randomize_size = randomize_btn.calc_size(ctx);
        sprites.push(Box::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        )));
        sprites.push(Box::new(randomize_btn));
        sprites.push(Box::new(Button::new(
            "create",
            vec![(Key::Enter, None)],
            "[Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        )));
        Self {
            assets,
            sprites,
            savefile,
        }
    }
}

impl Scene for CreateCharacter {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "back" => Some(Transition::Pop),
            _ => None,
        }
    }

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
