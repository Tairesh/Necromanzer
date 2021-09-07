mod dropping;
mod examining;
mod walking;

use action::{Action, ActionType};
use assets::Assets;
use colors::Colors;
use direction::Direction;
use enum_dispatch::enum_dispatch;
use human::gender::Gender;
use scenes::game::dropping::Dropping;
use scenes::game::examining::Examining;
use scenes::game::walking::Walking;
use scenes::game_menu::GameMenu;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::{graphics, input, window, Context};
use world::World;
use Vec2;

enum UpdateResult {
    DoNothing,
    OpenMenu,
    ClearLog,
    Examine(Direction),
    Drop(Direction),
    ResetGameMode,
    SwitchGameMode(GameMode),
    SetAvatarAction(ActionType),
}

#[enum_dispatch()]
trait GameModeTrait {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> UpdateResult;
    fn draw(&mut self, _ctx: &mut Context, _world: &mut World, _center: Vec2, _zoom: f32) {}
}

#[enum_dispatch(GameModeTrait)]
enum GameMode {
    Walking,
    Examining,
    Dropping,
}

pub struct Game {
    world: World,
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    zoom: i32,
    mode: GameMode,
    log: VecDeque<Text>,
}

impl Game {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        world: World,
        _ctx: &mut Context,
    ) -> Self {
        let hp_bar = Rc::new(RefCell::new(Bar::red(100, 50, assets.clone())));
        let mp_bar = Rc::new(RefCell::new(Bar::blue(100, 50, assets.clone())));
        let assets_copy = assets.clone();
        let assets = assets.borrow();
        let hat = Rc::new(RefCell::new(
            Image::new(assets.hat.clone(), Position::zeroed()).with_scale(Vec2::new(4.0, 4.0)),
        ));
        let name = Rc::new(RefCell::new(Label::new(
            world.avatar.character.name.as_str(),
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(174.0, 55.0, AnchorX::Center, AnchorY::Top),
        )));
        let ava = Rc::new(RefCell::new(Image::icon(
            assets.tileset.clone(),
            match world.avatar.character.gender {
                Gender::Female => assets.regions.female,
                Gender::Male => assets.regions.male,
                Gender::Custom(_) => assets.regions.queer,
            },
            Vec2::new(6.0, 6.0),
            world.avatar.character.skin_tone.color(),
            Position::new(52.0, 52.0, AnchorX::Center, AnchorY::Center),
        )));

        Self {
            sprites: vec![hat, name, ava, hp_bar, mp_bar],
            settings,
            world,
            assets: assets_copy,
            zoom: 2,
            mode: Walking::new().into(),
            log: VecDeque::new(),
        }
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        match self.mode.update(ctx, &mut self.world) {
            UpdateResult::DoNothing => {}
            UpdateResult::ClearLog => {
                self.log.clear();
            }
            UpdateResult::Examine(dir) => {
                if let Some(dir) = dir.as_two_dimensional() {
                    self.world.avatar.vision = dir;
                }
                let pos = self.world.avatar.pos.add(dir);
                let tile = self.world.load_tile_mut(pos);
                self.log.push_front(Text::new(
                    tile.terrain.this_is(),
                    self.assets.borrow().default.clone(),
                ));
                if let Some(item) = tile.items.pop() {
                    self.world.avatar.wield.push(item.clone());
                    self.log.push_front(Text::new(
                        format!("You wields {:?}", item),
                        self.assets.borrow().default.clone(),
                    ));
                }
            }
            UpdateResult::Drop(dir) => {
                let mut items = self.world.avatar.wield.clone();
                self.world.avatar.wield.clear();
                let tile = self.world.load_tile_mut(self.world.avatar.pos.add(dir));
                tile.items.append(&mut items);
            }
            UpdateResult::OpenMenu => {
                return Some(Transition::Push(Box::new(GameMenu::new(
                    self.assets.clone(),
                    self.settings.clone(),
                    ctx,
                ))));
            }
            UpdateResult::ResetGameMode => {
                self.mode = Walking::new().into();
            }
            UpdateResult::SwitchGameMode(mode) => {
                self.mode = mode;
            }
            UpdateResult::SetAvatarAction(action) => {
                if action.is_possible(&mut self.world) {
                    if action.length(&mut self.world) > 20.0 {
                        self.log.push_front(Text::new(
                            format!("It takes a long time to {}.", action.name(&mut self.world)),
                            self.assets.borrow().default.clone(),
                        ));
                    }
                    self.world.avatar.action = Some(Action::new(&mut self.world, action));
                }
            }
        }
        let scroll = input::get_mouse_wheel_movement(ctx).y;
        if scroll != 0 {
            self.zoom += scroll;
            if self.zoom < 1 {
                self.zoom = 1;
            } else if self.zoom > 10 {
                self.zoom = 10;
            }
        }
        self.world.tick();
        update_sprites(self, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::BLACK);
        let window_size = window::get_size(ctx);
        let zoom = self.zoom as f32;
        let scale = Vec2::new(zoom, zoom);
        let window_size_in_tiles = (
            (window_size.0 as f32 / (10.0 * zoom)).ceil() as i32,
            (window_size.1 as f32 / (10.0 * zoom as f32)).ceil() as i32,
        );
        let center = Vec2::new(
            window_size.0 as f32 / 2.0 - 5.0 * zoom,
            window_size.1 as f32 / 2.0 - 5.0 * zoom,
        );
        {
            let assets = self.assets.borrow();
            let center_tile = self.world.avatar.pos;
            let left_top =
                center_tile.add_delta(-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
            let right_bottom =
                center_tile.add_delta(window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
            for (pos, tile) in self.world.tiles_between(left_top, right_bottom).into_iter() {
                let dx = pos.x - center_tile.x;
                let dy = pos.y - center_tile.y;
                let region = tile.terrain.region(&assets.regions);
                let params = DrawParams::new()
                    .position(Vec2::new(
                        center.x + dx as f32 * 10.0 * zoom,
                        center.y + dy as f32 * 10.0 * zoom,
                    ))
                    .scale(scale);
                assets.tileset.draw_region(ctx, region, params.clone());
                if let Some(item) = tile.top_item() {
                    assets
                        .tileset
                        .draw_region(ctx, item.region(&assets.regions), params.clone());
                    if tile.items.len() > 1 {
                        assets
                            .tileset
                            .draw_region(ctx, assets.regions.highlight, params);
                    }
                }
            }
        }
        self.world
            .avatar
            .draw(ctx, self.assets.clone(), center, zoom);
        self.redraw_sprites(ctx);
        self.mode.draw(ctx, &mut self.world, center, zoom);
        for (i, text) in self.log.iter_mut().enumerate() {
            text.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        10.0,
                        window_size.1 as f32 - 20.0 - 20.0 * i as f32,
                    ))
                    .color(if i == 0 {
                        Colors::LIGHT_YELLOW
                    } else {
                        Colors::GRAY
                    }),
            );
            if i >= 5 {
                break;
            }
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.world.save();
    }
}
