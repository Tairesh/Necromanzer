use arr_macro::arr;
use assets::Assets;
use colors::Colors;
use maptile::{DirtVariant, TileBase};
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::{graphics, input, window, Context, TetraVec2};
use world::World;

pub struct Game {
    world: World,
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    tiles: [TileBase; 2048],
}

impl Game {
    pub fn new(assets: Rc<RefCell<Assets>>, world: World, _ctx: &mut Context) -> Self {
        let hp_bar = Rc::new(RefCell::new(Bar::red(100, 50, assets.clone())));
        let mp_bar = Rc::new(RefCell::new(Bar::blue(100, 50, assets.clone())));
        let assets_copy = assets.clone();
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

        let tiles = arr![TileBase::Dirt(rand::random::<DirtVariant>()); 2048];
        Self {
            sprites: vec![hat, name, ava, hp_bar, mp_bar],
            world,
            assets: assets_copy,
            tiles,
        }
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            self.world.save();
            Ok(Transition::Pop)
        } else if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::BLACK);
        {
            let assets = self.assets.borrow();
            for i in 0..2048 {
                let (x, y) = (i % 64, i / 64);
                let region = match &self.tiles[i] {
                    TileBase::Dirt(variant) => match variant {
                        DirtVariant::Dirt1 => assets.icons.dirt1,
                        DirtVariant::Dirt2 => assets.icons.dirt2,
                        DirtVariant::Dirt3 => assets.icons.dirt3,
                        DirtVariant::Dirt4 => assets.icons.dirt4,
                        DirtVariant::Dirt5 => assets.icons.dirt5,
                    },
                };
                assets.tileset.draw_region(
                    ctx,
                    region,
                    DrawParams::new()
                        .position(TetraVec2::new(x as f32 * 30.0, y as f32 * 30.0))
                        .scale(TetraVec2::new(3.0, 3.0)),
                )
            }
            let (w, h) = window::get_size(ctx);
            assets.tileset.draw_region(
                ctx,
                match self.world.avatar.gender.as_str() {
                    "Female" => assets.icons.female,
                    "Male" => assets.icons.male,
                    _ => assets.icons.queer,
                },
                DrawParams::new()
                    .position(TetraVec2::new(w as f32 / 2.0 - 15.0, h as f32 / 2.0 - 15.0))
                    .scale(TetraVec2::new(3.0, 3.0))
                    .color(self.world.avatar.skin_tone.color()),
            );
        }
        self.redraw_sprites(ctx)?;
        Ok(())
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
