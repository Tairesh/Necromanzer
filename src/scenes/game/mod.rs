mod dropping;
mod examining;
mod walking;

use action::{Action, ActionType};
use assets::Assets;
use colors::Colors;
use direction::Direction;
use enum_dispatch::enum_dispatch;
use human::gender::Gender;
use itertools::Itertools;
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
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::{graphics, input, window, Context};
use world::World;
use Vec2;

enum UpdateResult {
    OpenMenu,
    ClearLog,
    Examine(Direction),
    Drop(Direction),
    ResetGameMode,
    SwitchGameMode(GameMode),
    SetAvatarAction(ActionType),
    AddLogMessage(String),
}

#[enum_dispatch()]
trait GameModeTrait {
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Vec<UpdateResult>;
    fn draw(&mut self, _ctx: &mut Context, _world: &mut World, _center: Vec2, _zoom: f32) {}
}

#[enum_dispatch(GameModeTrait)]
enum GameMode {
    Walking,
    Examining,
    Dropping,
}

struct LogMessage {
    pub text: Text,
    color: Color,
}

impl LogMessage {
    pub fn new(text: &str, font: Font, color: Color) -> Self {
        Self {
            text: Text::new(text, font),
            color,
        }
    }
}

pub struct Game {
    world: World,
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    zoom: i32,
    mode: GameMode,
    log: VecDeque<LogMessage>,
}

impl Game {
    const LOG_LIMIT: usize = 5;

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
            log: VecDeque::with_capacity(Self::LOG_LIMIT),
        }
    }

    fn log(&mut self, text: &str) {
        if self.log.len() >= Self::LOG_LIMIT {
            self.log.pop_back();
        }
        self.log.push_front(LogMessage::new(
            text,
            self.assets.borrow().default.clone(),
            Colors::LIGHT_YELLOW,
        ));
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        for upd in self.mode.update(ctx, &mut self.world) {
            match upd {
                UpdateResult::ClearLog => {
                    self.log.clear();
                }
                UpdateResult::Examine(dir) => {
                    // TODO: move this to Examining gamemode
                    if let Some(dir) = dir.as_two_dimensional() {
                        self.world.avatar.vision = dir;
                    }
                    let pos = self.world.avatar.pos.add(dir);
                    let tile = self.world.load_tile(pos);
                    let mut this_is = tile.terrain.this_is();
                    if !tile.items.is_empty() {
                        #[allow(unstable_name_collisions)]
                        let items: String = tile
                            .items
                            .iter()
                            .map(|item| item.name())
                            .intersperse(", ")
                            .collect();
                        this_is += " Here you see: ";
                        this_is += items.as_str();
                    }
                    self.log(this_is.as_str());
                    // let tile = self.world.load_tile_mut(pos);
                    // if let Some(item) = tile.items.pop() {
                    //     self.world.avatar.wield.push(item.clone());
                    //     self.log(format!("You picked up a {:?}", item).as_str());
                    // }
                }
                UpdateResult::Drop(dir) => {
                    // TODO: move this to Dropping gamemode and make an Action (for time consuming)
                    let mut items = self.world.avatar.wield.clone();
                    if items.is_empty() {
                        self.log("You have nothing to drop!");
                    } else {
                        if items.len() == 1 {
                            self.log(
                                format!("You threw away the {:?}.", items.first().unwrap())
                                    .as_str(),
                            );
                        } else {
                            self.log(
                                format!("You threw some things ({}) away.", items.len()).as_str(),
                            );
                        }
                        self.world.avatar.wield.clear();
                        let tile = self.world.load_tile_mut(self.world.avatar.pos.add(dir));
                        tile.items.append(&mut items);
                    }
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
                    // TODO: Move this to Walking gamemode
                    if action.is_possible(&mut self.world) {
                        if action.length(&mut self.world) > 20.0 {
                            let text = format!(
                                "It takes a long time to {}.",
                                action.name(&mut self.world)
                            );
                            self.log(text.as_str());
                        }
                        self.world.avatar.action = Some(Action::new(&mut self.world, action));
                    }
                }
                UpdateResult::AddLogMessage(msg) => {
                    self.log(msg.as_str());
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
                        .draw_region(ctx, item.region(), params.clone());
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
        for (i, msg) in self.log.iter_mut().enumerate() {
            msg.text.draw(
                ctx,
                DrawParams::new()
                    .position(Vec2::new(
                        10.0,
                        window_size.1 as f32 - 20.0 - 20.0 * i as f32,
                    ))
                    .color(if i == 0 { msg.color } else { Colors::GRAY }),
            );
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
