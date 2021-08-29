use assets::Assets;
use colors::Colors;
use human::character::Character;
use savefile::SaveFile;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::MouseButton;
use tetra::{input, Context, TetraVec2};

pub struct Game {
    sprites: Vec<Box<dyn Sprite>>,
}

impl Game {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        _savefile: SaveFile,
        avatar: Character,
        _ctx: &mut Context,
    ) -> Self {
        let assets = assets.borrow();
        let hat =
            Image::new(assets.hat.clone(), Position::zeroed()).with_scale(TetraVec2::new(4.0, 4.0));
        let name = Label::new(
            avatar.name.as_str(),
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(174.0, 55.0, AnchorX::Center, AnchorY::Top),
        );
        let ava = Image::icon(
            assets.tileset.clone(),
            match avatar.gender.as_str() {
                "Female" => assets.icons.female,
                "Male" => assets.icons.male,
                _ => assets.icons.queer,
            },
            TetraVec2::new(6.0, 6.0),
            avatar.skin_tone.color(),
            Position::new(52.0, 52.0, AnchorX::Center, AnchorY::Center),
        );
        let hp_bar = Image::new(
            assets.bars.clone(),
            Position::new(100.0, 8.0, AnchorX::Left, AnchorY::Top),
        )
        .with_scale(TetraVec2::new(4.0, 4.0))
        .with_nineslice(assets.bar_red.clone(), 50.0, 3.0);
        let mp_bar = Image::new(
            assets.bars.clone(),
            Position::new(100.0, 32.0, AnchorX::Left, AnchorY::Top),
        )
        .with_scale(TetraVec2::new(4.0, 4.0))
        .with_nineslice(assets.bar_blue.clone(), 43.0, 3.0);
        Self {
            sprites: vec![
                Box::new(hat),
                Box::new(name),
                Box::new(ava),
                Box::new(hp_bar),
                Box::new(mp_bar),
            ],
        }
    }
}

impl Scene for Game {
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
