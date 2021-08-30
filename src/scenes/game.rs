use assets::Assets;
use colors::Colors;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::MouseButton;
use tetra::{input, Context, TetraVec2};
use world::World;

pub struct Game {
    world: World,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    hp_bar: Rc<RefCell<Bar>>,
}

impl Game {
    pub fn new(assets: Rc<RefCell<Assets>>, world: World, _ctx: &mut Context) -> Self {
        let hp_bar = Rc::new(RefCell::new(Bar::red(100, 50, assets.clone())));
        let mp_bar = Rc::new(RefCell::new(Bar::blue(100, 50, assets.clone())));
        let assets = assets.borrow();
        let hat = Rc::new(RefCell::new(
            Image::new(assets.hat.clone(), Position::zeroed()).with_scale(TetraVec2::new(4.0, 4.0)),
        ));
        let name = Rc::new(RefCell::new(Label::new(
            world.avatar.name.as_str(),
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(174.0, 55.0, AnchorX::Center, AnchorY::Top),
        )));
        let ava = Rc::new(RefCell::new(Image::icon(
            assets.tileset.clone(),
            match world.avatar.gender.as_str() {
                "Female" => assets.icons.female,
                "Male" => assets.icons.male,
                _ => assets.icons.queer,
            },
            TetraVec2::new(6.0, 6.0),
            world.avatar.skin_tone.color(),
            Position::new(52.0, 52.0, AnchorX::Center, AnchorY::Center),
        )));

        Self {
            sprites: vec![hat, name, ava, hp_bar.clone(), mp_bar],
            world,
            hp_bar,
        }
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let val = self.hp_bar.borrow().value();
        self.hp_bar.borrow_mut().set_value(val + 1);
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            self.world.save();
            Ok(Transition::Pop)
        } else if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
