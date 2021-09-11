use action::{Action, ActionType};
use assets::Assets;
use colors::Colors;
use direction::Direction;
use geometry::DIR9;
use human::main_hand::MainHand;
use itertools::Itertools;
use map::terrains::Terrain;
use map::tile::Tile;
use scenes::game::menu::Menu;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::alert::Alert;
use sprites::image::{Bar, Image};
use sprites::label::{ItemDisplay, Label};
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::{Sprite, Stringify};
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
    Dropping,
    Digging,
}

impl GameMode {
    pub fn draw_cursors(&self) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Examining | GameMode::Wielding | GameMode::Dropping | GameMode::Digging => {
                true
            }
        }
    }

    pub fn cursor_here(&self, tile: &Tile) -> bool {
        match self {
            GameMode::Wielding => !tile.items.is_empty(),
            GameMode::Dropping => tile.terrain.is_walkable(),
            GameMode::Digging => matches!(tile.terrain, Terrain::Grave(..)),
            GameMode::Examining | GameMode::Default => false,
        }
    }
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
    item_display: Rc<RefCell<ItemDisplay>>,
    action_text: Option<Text>,
    current_time: Rc<RefCell<Label>>,
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
        let bg = Rc::new(RefCell::new(
            Alert::new(
                250.0,
                90.0,
                assets.clone(),
                Position::by_left_top(0.0, 68.0),
            )
            .with_scale(Vec2::new(4.0, 4.0)),
        ));
        let assets_copy = assets.clone();
        let assets = assets.borrow();
        let hat = Rc::new(RefCell::new(
            Image::new(assets.hat.clone(), Position::zeroed()).with_scale(Vec2::new(4.0, 4.0)),
        ));
        let name_label = Rc::new(RefCell::new(Label::new(
            world.avatar.character.name.as_str(),
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::new(174.0, 55.0, AnchorX::Center, AnchorY::Top),
        )));
        let hands_label = Rc::new(RefCell::new(Label::new(
            if matches!(world.avatar.character.main_hand, MainHand::Left) {
                "Left hand:\nRight hand:"
            } else {
                "Right hand:\nLeft hand:"
            },
            assets.default.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_left_top(30.0, 98.0),
        )));
        let item_display = Rc::new(RefCell::new(ItemDisplay::new(
            world.avatar.wield.get(0),
            assets.default.clone(),
            Colors::LIGHT_YELLOW,
            assets.tileset.clone(),
            Vec2::new(2.0, 2.0),
            Position::by_right_top(220.0, 98.0),
        )));
        let current_time = Rc::new(RefCell::new(Label::new(
            format!("{}", world.meta.current_tick),
            assets.default2.clone(),
            Colors::LIGHT_YELLOW,
            Position::by_right_top(window::get_width(ctx) as f32 - 5.0, 0.0),
        )));

        Self {
            sprites: vec![
                bg,
                hat,
                name_label,
                hands_label,
                item_display.clone(),
                hp_bar,
                mp_bar,
                current_time.clone(),
            ],
            item_display,
            current_time,
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
            action_text: None,
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

    fn select(&mut self, dir: Direction) {
        if self.selected.is_none() {
            self.selected = Some(dir);
            if let Some(dir) = dir.as_two_dimensional() {
                self.world.borrow_mut().avatar.vision = dir;
            }
        }
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
                let mut world = self.world.borrow_mut();
                let action = ActionType::Dropping(Direction::Here);
                if action.is_possible(&mut world) {
                    let length = action.length(&mut world);
                    let finish = world.meta.current_tick + length;
                    world.avatar.action = Some(Action::new(finish, action));
                } else {
                    drop(world);
                    self.log("You can't put items here!");
                }
            }
        } else if input::is_key_pressed(ctx, Key::D)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            if self.world.borrow().avatar.wield.is_empty() {
                self.log("You have nothing to drop!");
            } else {
                self.mode = GameMode::Dropping;
            }
        } else if input::is_key_pressed(ctx, Key::W) && input::is_no_key_modifiers(ctx) {
            self.mode = GameMode::Wielding;
        } else if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            self.log.clear();
        } else if input::is_key_pressed(ctx, Key::G) && input::is_no_key_modifiers(ctx) {
            self.mode = GameMode::Digging;
        }
        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(self.last_walk).as_millis() > 75
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                let mut world = self.world.borrow_mut();
                if dir.is_here() {
                    let action = ActionType::SkippingTime;
                    let finish = world.meta.current_tick + action.length(&mut world);
                    world.avatar.action = Some(Action::new(finish, action));
                } else {
                    let action = ActionType::Walking(dir);
                    if action.is_possible(&mut world) {
                        let length = action.length(&mut world);
                        let finish = world.meta.current_tick + length;
                        world.avatar.action = Some(Action::new(finish, action));
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
                self.select(dir);
                let mut world = self.world.borrow_mut();
                let pos = world.avatar.pos + dir;
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
            self.select(dir);
        } else if let Some(dir) = self.selected {
            let action = ActionType::Wielding(dir);
            let mut world = self.world.borrow_mut();
            if action.is_possible(&mut world) {
                let length = action.length(&mut world);
                let finish = world.meta.current_tick + length;
                world.avatar.action = Some(Action::new(finish, action));
            } else {
                drop(world);
                self.log("Nothing to pick up here!");
            }
            self.mode = GameMode::Default;
            self.selected = None;
        }
        None
    }

    fn update_dropping(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.select(dir);
        } else if let Some(dir) = self.selected {
            let action = ActionType::Dropping(dir);
            let mut world = self.world.borrow_mut();
            if action.is_possible(&mut world) {
                let length = action.length(&mut world);
                let finish = world.meta.current_tick + length;
                world.avatar.action = Some(Action::new(finish, action));
            } else {
                drop(world);
                self.log("You can't drop items here!");
            }
            self.mode = GameMode::Default;
            self.selected = None;
        }
        None
    }

    fn update_digging(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_key_pressed(ctx, Key::Escape) {
            self.mode = GameMode::Default;
            self.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.select(dir);
        } else if let Some(dir) = self.selected {
            let action = ActionType::Digging(dir);
            let mut world = self.world.borrow_mut();
            if action.is_possible(&mut world) {
                let length = action.length(&mut world);
                let finish = world.meta.current_tick + length;
                world.avatar.action = Some(Action::new(finish, action));
            } else {
                drop(world);
                self.log("You can't dig here!");
            }
            self.mode = GameMode::Default;
            self.selected = None;
        }

        None
    }

    fn draw_action_loader(&mut self, ctx: &mut Context, center: Vec2) {
        if self.action_text.is_none() {
            let text = Text::new(
                self.world.borrow().avatar.action.unwrap().action.verb(),
                self.assets.borrow().default.clone(),
            );
            self.action_text = Some(text);
        }
        let text = self.action_text.as_mut().unwrap();
        let bounds = text.get_bounds(ctx).unwrap();
        text.draw(
            ctx,
            center
                - Vec2::new(
                    bounds.width / 2.0 - Assets::TILE_SIZE as f32 * self.zoom as f32 / 2.0,
                    bounds.height,
                ),
        );
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
            GameMode::Dropping => self.update_dropping(ctx),
            GameMode::Digging => self.update_digging(ctx),
        } {
            return Some(t);
        }
        if self.world.borrow_mut().avatar.action.is_some() {
            let (delta, action) = {
                let mut world = self.world.borrow_mut();
                let starting_tick = world.meta.current_tick;
                let action = world.avatar.action.unwrap().action.name(&mut world);
                world.tick();
                ((world.meta.current_tick - starting_tick) as u32, action)
            };
            if delta > 20 && delta < World::SPEND_LIMIT {
                self.log(format!("It takes a long time to {}.", action).as_str());
            }
            self.item_display
                .borrow_mut()
                .set_item(self.world.borrow_mut().avatar.wield.first(), ctx);
            self.current_time
                .borrow_mut()
                .set_value(format!("{}", self.world.borrow().meta.current_tick));
        }
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
            let left_top = center_tile + (-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
            let right_bottom =
                center_tile + (window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
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
            .draw(ctx, &self.assets.borrow().tileset, center, zoom, true);
        if self.world.borrow().avatar.action.is_some() {
            self.draw_action_loader(ctx, center);
        } else {
            self.action_text = None;
        }
        if self.mode.draw_cursors() {
            let mut world = self.world.borrow_mut();
            for (dx, dy) in DIR9 {
                let pos = world.avatar.pos + (dx, dy);
                if self.mode.cursor_here(world.load_tile(pos)) {
                    let delta = Vec2::new(dx as f32, dy as f32) * Assets::TILE_SIZE as f32 * zoom;
                    self.cursor.draw(
                        ctx,
                        DrawParams::new()
                            .position(center + delta)
                            .scale(scale)
                            .color(Colors::LIGHT_GREEN.with_alpha(0.7)),
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
        self.world.borrow().avatar.draw(
            ctx,
            &self.assets.borrow().tileset,
            Vec2::new(20.0, 20.0),
            6.0,
            false,
        );
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
