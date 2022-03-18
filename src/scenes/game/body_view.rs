use assets::Assets;
use colors::Colors;
use human::body::Body;
use human::character::Character;
use input;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::alert::Alert;
use sprites::label::Label;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::Context;

pub struct BodyView {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl BodyView {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        character: &Character,
        body: &Body,
        _ctx: &mut Context,
    ) -> Self {
        let assets = assets.borrow();
        let mut sprites: Vec<Rc<RefCell<dyn Sprite>>> =
            Vec::with_capacity(body.parts.len() * 2 + 2);
        sprites.push(Rc::new(RefCell::new(Alert::new(
            600.0,
            400.0,
            assets.alert_asset.texture.clone(),
            assets.alert_asset.nineslice.clone(),
            Position::center(),
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            format!(
                "{} ({}, {})",
                &character.name, &character.gender, &character.age
            ),
            assets.fonts.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -280.0 },
                y: Vertical::AtWindowCenterByTop { offset: -180.0 },
            },
        ))));
        let mut y_center = -180.0;
        let mut y_left = -180.0;
        let mut y_right = -180.0;
        for (key, part) in body.parts.iter() {
            let (x, y) = if key.starts_with("left") {
                y_left += 50.0;
                (Horizontal::AtWindowCenterByLeft { offset: -270.0 }, y_left)
            } else if key.starts_with("right") {
                y_right += 50.0;
                (Horizontal::AtWindowCenterByRight { offset: 270.0 }, y_right)
            } else {
                y_center += 50.0;
                (Horizontal::AtWindowCenter { offset: 0.0 }, y_center)
            };
            sprites.push(Rc::new(RefCell::new(Label::new(
                key,
                assets.fonts.default.clone(),
                Colors::LIGHT_GREEN,
                Position {
                    x,
                    y: Vertical::AtWindowCenterByTop { offset: y },
                },
            ))));
            if let Some(body_part) = part.item_type.body_part() {
                sprites.push(Rc::new(RefCell::new(Label::new(
                    part.item_type.name(),
                    assets.fonts.default2.clone(),
                    body_part.skin_tone.into(),
                    Position {
                        x,
                        y: Vertical::AtWindowCenterByTop { offset: y + 20.0 },
                    },
                ))));
            }
        }
        Self { sprites }
    }
}

impl Scene for BodyView {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1)
            || input::is_key_pressed(ctx, Key::Escape)
        {
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
