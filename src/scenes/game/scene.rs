use action::{Action, ActionType};
use assets::Assets;
use colors::Colors;
use direction::Direction;
use geometry::DIR9;
use human::gender::Gender;
use itertools::Itertools;
use scenes::game::menu::Menu;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Instant;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier};
use tetra::{graphics, window, Context};
use world::World;
use {input, Vec2};

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

#[derive(Debug)]
enum GameMode {
    Default,
    Examining,
    Wielding,
}

pub struct Game {
    world: Rc<RefCell<World>>,
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    zoom: i32,
    log: VecDeque<LogMessage>, // TODO: separate object Rc<RefCell<Logger>>
    last_walk: Instant,
    mode: GameMode,
    cursor: Mesh,
    selected: Option<Direction>,
}

impl Game {
    const LOG_LIMIT: usize = 5;

    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        world: World,
        ctx: &mut Context,
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
            world: Rc::new(RefCell::new(world)),
            assets: assets_copy,
            zoom: 2,
            log: VecDeque::with_capacity(Self::LOG_LIMIT),
            last_walk: Instant::now(),
            mode: GameMode::Default,
            cursor: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(0.0, 0.0, Assets::TILE_SIZE as f32, Assets::TILE_SIZE as f32),
            )
            .unwrap(),
            selected: None,
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

    fn update_default(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            return Some(Transition::Push(Box::new(Menu::new(
                self.assets.clone(),
                self.settings.clone(),
                ctx,
            ))));
        } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
            self.mode = GameMode::Examining;
        } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
            if self.world.borrow().avatar.wield.is_empty() {
                self.log("You have nothing to drop!");
            } else {
                let pos = self.world.borrow().avatar.pos;
                let mut world = self.world.borrow_mut();
                let item = world.avatar.wield.pop().unwrap();
                let name = item.name().to_string();
                world.load_tile_mut(pos).items.push(item);
                drop(world);
                self.log(format!("You threw away the {}.", name).as_str());
            }
        } else if input::is_key_pressed(ctx, Key::W) && input::is_no_key_modifiers(ctx) {
            // return Some(Transition::Push(Box::new(Wielding::new(
            //     self.assets.clone(),
            //     self.world.clone(),
            //     ctx,
            // ))));
            self.mode = GameMode::Wielding;
        } else if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            self.log.clear();
        }
        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(self.last_walk).as_millis() > 75
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                if dir.is_here() {
                    let finish = self.world.borrow().meta.current_tick + 1.0;
                    self.world.borrow_mut().avatar.action =
                        Some(Action::new(finish, ActionType::SkippingTime));
                } else {
                    let action = ActionType::Walking(dir);
                    if action.is_possible(self.world.clone()) {
                        let length = action.length(self.world.clone());
                        if length > 20.0 {
                            let text = format!(
                                "It takes a long time to {}.",
                                action.name(self.world.clone())
                            );
                            self.log(text.as_str());
                        }
                        let finish = self.world.borrow().meta.current_tick + length;
                        self.world.borrow_mut().avatar.action = Some(Action::new(finish, action));
                    }
                }
            }
        }
        None
    }
    fn update_examining(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if self.selected.is_none() {
                self.selected = Some(dir);
                if let Some(dir) = dir.as_two_dimensional() {
                    self.world.borrow_mut().avatar.vision = dir;
                }
                let pos = self.world.borrow().avatar.pos.add(dir);
                let mut world = self.world.borrow_mut();
                let tile = world.load_tile(pos);
                let mut this_is = tile.terrain.this_is();
                if !tile.items.is_empty() {
                    // TODO: use the std version when stable (see https://github.com/rust-lang/rust/issues/79524)
                    let items: String =
                        Itertools::intersperse(tile.items.iter().map(|item| item.name()), ", ")
                            .collect();
                    this_is += " Here you see: ";
                    this_is += items.as_str();
                }
                drop(world);
                self.log(this_is.as_str());
            }
        } else if self.selected.is_some() {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        None
    }
    fn update_wielding(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if self.selected.is_none() {
                self.selected = Some(dir);
                if let Some(dir) = dir.as_two_dimensional() {
                    self.world.borrow_mut().avatar.vision = dir;
                }
                let pos = self.world.borrow().avatar.pos.add(dir);
                let mut world = self.world.borrow_mut();
                let msg = if let Some(item) = world.load_tile_mut(pos).items.pop() {
                    let msg = format!("You have picked up the {}.", item.name());
                    world.avatar.wield.push(item);
                    msg
                } else {
                    "Nothing to pick up here!".to_string()
                };
                drop(world);
                self.log(msg.as_str());
            }
        } else if self.selected.is_some() {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        None
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        let scroll = input::get_mouse_wheel_movement(ctx).y;
        if scroll != 0 {
            self.zoom += scroll;
            if self.zoom < 1 {
                self.zoom = 1;
            } else if self.zoom > 10 {
                self.zoom = 10;
            }
        }
        if let Some(t) = match self.mode {
            GameMode::Default => self.update_default(ctx),
            GameMode::Examining => self.update_examining(ctx),
            GameMode::Wielding => self.update_wielding(ctx),
        } {
            return Some(t);
        }
        self.world.borrow_mut().tick();
        update_sprites(self, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::BLACK);
        let window_size = window::get_size(ctx);
        let zoom = self.zoom as f32;
        let scale = Vec2::new(zoom, zoom);
        let window_size_in_tiles = (
            (window_size.0 as f32 / (Assets::TILE_SIZE as f32 * zoom)).ceil() as i32,
            (window_size.1 as f32 / (Assets::TILE_SIZE as f32 * zoom as f32)).ceil() as i32,
        );
        let center = Vec2::new(
            window_size.0 as f32 / 2.0 - 5.0 * zoom,
            window_size.1 as f32 / 2.0 - 5.0 * zoom,
        );
        {
            let assets = self.assets.borrow();
            let center_tile = self.world.borrow().avatar.pos;
            let left_top =
                center_tile.add_delta(-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
            let right_bottom =
                center_tile.add_delta(window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
            for (pos, tile) in self
                .world
                .borrow_mut()
                .tiles_between(left_top, right_bottom)
                .into_iter()
            {
                let dx = pos.x - center_tile.x;
                let dy = pos.y - center_tile.y;
                let region = tile.terrain.region(&assets.regions);
                let params = DrawParams::new()
                    .position(Vec2::new(
                        center.x + dx as f32 * Assets::TILE_SIZE as f32 * zoom,
                        center.y + dy as f32 * Assets::TILE_SIZE as f32 * zoom,
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
            .borrow()
            .avatar
            .draw(ctx, self.assets.clone(), center, zoom);
        if let GameMode::Wielding = self.mode {
            for (dx, dy) in DIR9 {
                let pos = self.world.borrow().avatar.pos.add_delta(dx, dy);
                let mut world = self.world.borrow_mut();
                let tile = world.load_tile(pos);
                if !tile.items.is_empty() {
                    let delta = Vec2::new(dx as f32, dy as f32) * Assets::TILE_SIZE as f32 * zoom;
                    self.cursor.draw(
                        ctx,
                        DrawParams::new()
                            .position(center + delta)
                            .scale(scale)
                            .color(Colors::LIGHT_CORAL.with_alpha(0.7)),
                    );
                }
            }
        }
        if let Some(dir) = self.selected {
            let delta =
                Vec2::new(dir.dx() as f32, dir.dy() as f32) * Assets::TILE_SIZE as f32 * zoom;
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .scale(scale)
                    .position(center + delta)
                    .color(Colors::LIGHT_YELLOW.with_alpha(0.7)),
            )
        }
        self.redraw_sprites(ctx);
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
        self.world.borrow_mut().save();
    }
}
