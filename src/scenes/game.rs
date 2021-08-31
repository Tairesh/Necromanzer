use assets::Assets;
use colors::Colors;
use maptile::{BoulderVariant, DirtVariant, TileBase};
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::DrawParams;
use tetra::input::{Key, MouseButton};
use tetra::{graphics, input, window, Context, TetraVec2};
use world::World;

pub struct Game {
    world: World,
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    zoom: f32,
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
            world.avatar.character.name.as_str(),
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(174.0, 55.0, AnchorX::Center, AnchorY::Top),
        )));
        let ava = Rc::new(RefCell::new(Image::icon(
            assets.tileset.clone(),
            match world.avatar.character.gender.as_str() {
                "Female" => assets.icons.female,
                "Male" => assets.icons.male,
                _ => assets.icons.queer,
            },
            TetraVec2::new(6.0, 6.0),
            world.avatar.character.skin_tone.color(),
            Position::new(52.0, 52.0, AnchorX::Center, AnchorY::Center),
        )));

        Self {
            sprites: vec![hat, name, ava, hp_bar, mp_bar],
            world,
            assets: assets_copy,
            zoom: 2.0,
        }
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        let scroll = input::get_mouse_wheel_movement(ctx).y;
        if scroll != 0 {
            self.zoom += scroll as f32;
            if self.zoom < 1.0 {
                self.zoom = 1.0;
            } else if self.zoom > 10.0 {
                self.zoom = 10.0;
            }
        }
        if input::is_key_down(ctx, Key::Up) {
            self.world.avatar.pos.translate(0, -1);
        }
        if input::is_key_down(ctx, Key::Down) {
            self.world.avatar.pos.translate(0, 1);
        }
        if input::is_key_down(ctx, Key::Left) {
            self.world.avatar.pos.translate(-1, 0);
        }
        if input::is_key_down(ctx, Key::Right) {
            self.world.avatar.pos.translate(1, 0);
        }
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            self.world.save();
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::BLACK);
        {
            let assets = self.assets.borrow();
            let window_size = window::get_size(ctx);
            let window_size_in_tiles = (
                (window_size.0 as f32 / (10.0 * self.zoom)).ceil() as i32,
                (window_size.1 as f32 / (10.0 * self.zoom)).ceil() as i32,
            );
            let center = (
                window_size.0 as f32 / 2.0 - 5.0 * self.zoom,
                window_size.1 as f32 / 2.0 - 5.0 * self.zoom,
            );
            for x in (-window_size_in_tiles.0 / 2)..=(window_size_in_tiles.0 / 2) {
                for y in (-window_size_in_tiles.1 / 2)..=(window_size_in_tiles.1 / 2) {
                    let tile = self.world.load_tile(self.world.avatar.pos.add(x, y));
                    let region = match tile {
                        TileBase::Dirt(variant) => match variant {
                            DirtVariant::Dirt1 => assets.icons.dirt1,
                            DirtVariant::Dirt2 => assets.icons.dirt2,
                            DirtVariant::Dirt3 => assets.icons.dirt3,
                            DirtVariant::Dirt4 => assets.icons.dirt4,
                            DirtVariant::Dirt5 => assets.icons.dirt5,
                        },
                        TileBase::Boulder(variant) => match variant {
                            BoulderVariant::One1 => assets.icons.boulder1,
                            BoulderVariant::One2 => assets.icons.boulder2,
                            BoulderVariant::One3 => assets.icons.boulder3,
                            BoulderVariant::Two1 => assets.icons.boulders1,
                            BoulderVariant::Two2 => assets.icons.boulders2,
                            BoulderVariant::Three1 => assets.icons.boulders3,
                            BoulderVariant::Three2 => assets.icons.boulders4,
                        },
                    };
                    assets.tileset.draw_region(
                        ctx,
                        region,
                        DrawParams::new()
                            .position(TetraVec2::new(
                                center.0 + x as f32 * 10.0 * self.zoom,
                                center.1 + y as f32 * 10.0 * self.zoom,
                            ))
                            .scale(TetraVec2::new(self.zoom, self.zoom)),
                    )
                }
            }

            assets.tileset.draw_region(
                ctx,
                match self.world.avatar.character.gender.as_str() {
                    "Female" => assets.icons.female,
                    "Male" => assets.icons.male,
                    _ => assets.icons.queer,
                },
                DrawParams::new()
                    .position(TetraVec2::new(
                        center.0 + 10.0 * self.zoom, // due to negative x-scale
                        center.1,
                    ))
                    .scale(TetraVec2::new(-self.zoom, self.zoom))
                    .color(self.world.avatar.character.skin_tone.color()),
            );
        }
        self.redraw_sprites(ctx);
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
