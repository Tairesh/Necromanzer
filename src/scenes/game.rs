use action::{Action, ActionType};
use assets::Assets;
use colors::Colors;
use direction::Direction;
use human::gender::Gender;
use input::{get_direction_keys_down, is_no_key_modifiers};
use scenes::game_menu::GameMenu;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::image::{Bar, Image};
use sprites::label::Label;
use sprites::meshy::JustMesh;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::{Draw, Positionate, Sprite};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Sub;
use std::rc::Rc;
use std::time::{Duration, Instant};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier};
use tetra::{graphics, input, window, Context, TetraVec2};
use world::World;

#[derive(Debug)]
enum GameMode {
    Walking(Instant),
    Examining(Option<Direction>),
}

pub struct Game {
    world: World,
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    zoom: f32,
    mode: GameMode,
    examination_text: Label,
    cursor: JustMesh,
    log: VecDeque<Text>,
}

impl Game {
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
            match world.avatar.character.gender {
                Gender::Female => assets.regions.female,
                Gender::Male => assets.regions.male,
                Gender::Custom(_) => assets.regions.queer,
            },
            TetraVec2::new(6.0, 6.0),
            world.avatar.character.skin_tone.color(),
            Position::new(52.0, 52.0, AnchorX::Center, AnchorY::Center),
        )));
        let examination_text = Label::new(
            "Use moving keys to select tile for examination",
            assets.header2.clone(),
            Colors::LIGHT_YELLOW,
            Position::horizontal_center(0.0, 200.0, AnchorY::Top),
        );
        let cursor = JustMesh::new(
            Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 10.0, 10.0)).unwrap(),
            Some(Colors::LIGHT_YELLOW.with_alpha(0.2)),
            TetraVec2::new(10.0, 10.0),
            Position::zeroed(),
        );

        Self {
            sprites: vec![hat, name, ava, hp_bar, mp_bar],
            settings,
            world,
            assets: assets_copy,
            zoom: 2.0,
            mode: GameMode::Walking(Instant::now().sub(Duration::from_secs(1))),
            examination_text,
            cursor,
            log: VecDeque::new(),
        }
    }
}

impl Scene for Game {
    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if let GameMode::Walking(_) = self.mode {
            if input::is_key_pressed(ctx, Key::Escape) {
                self.world.save();
                return Some(Transition::Push(Box::new(GameMenu::new(
                    self.assets.clone(),
                    self.settings.clone(),
                    ctx,
                ))));
            }
            if input::is_key_pressed(ctx, Key::C)
                && input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.log.clear();
            }
        }
        let scroll = input::get_mouse_wheel_movement(ctx).y;
        if scroll != 0 {
            self.zoom += scroll as f32;
            if self.zoom < 1.0 {
                self.zoom = 1.0;
            } else if self.zoom > 10.0 {
                self.zoom = 10.0;
            }
        }
        match self.mode {
            GameMode::Walking(last_tick) => {
                let now = Instant::now();
                if let Some(dir) = get_direction_keys_down(ctx) {
                    if now.duration_since(last_tick).as_millis() > 100 {
                        if dir.is_here() {
                            self.world.avatar.action =
                                Some(Action::new(&self.world, ActionType::SkippingTime));
                        } else {
                            let action = ActionType::Walking(dir);
                            if action.is_possible(&mut self.world) {
                                self.world.avatar.action = Some(Action::new(&self.world, action));
                            }
                        }
                        self.mode = GameMode::Walking(now);
                    }
                }
                if input::is_key_pressed(ctx, Key::E) && is_no_key_modifiers(ctx) {
                    self.mode = GameMode::Examining(None);
                }
            }
            GameMode::Examining(dir) => {
                if let Some(dir) = dir {
                    if get_direction_keys_down(ctx).is_none() {
                        self.mode = GameMode::Walking(Instant::now());
                        let tile = self.world.avatar.pos.add(dir);
                        self.log.push_front(Text::new(
                            self.world.load_tile(tile).terrain.this_is(),
                            self.assets.borrow().default.clone(),
                        ))
                    }
                } else if let Some(dir) = get_direction_keys_down(ctx) {
                    self.mode = GameMode::Examining(Some(dir));
                }
                if input::is_key_pressed(ctx, Key::Escape) {
                    self.mode = GameMode::Walking(Instant::now());
                }
            }
        }
        self.world.tick();
        update_sprites(self, ctx)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::BLACK);
        let window_size = window::get_size(ctx);
        let window_size_in_tiles = (
            (window_size.0 as f32 / (10.0 * self.zoom)).ceil() as i32,
            (window_size.1 as f32 / (10.0 * self.zoom)).ceil() as i32,
        );
        let center = TetraVec2::new(
            window_size.0 as f32 / 2.0 - 5.0 * self.zoom,
            window_size.1 as f32 / 2.0 - 5.0 * self.zoom,
        );
        {
            let assets = self.assets.borrow();
            for dx in (-window_size_in_tiles.0 / 2)..=(window_size_in_tiles.0 / 2) {
                for dy in (-window_size_in_tiles.1 / 2)..=(window_size_in_tiles.1 / 2) {
                    let pos = self.world.avatar.pos.add_delta(dx, dy);
                    let tile = self.world.load_tile(pos);
                    let region = tile.terrain.region(&assets.regions);
                    assets.tileset.draw_region(
                        ctx,
                        region,
                        DrawParams::new()
                            .position(TetraVec2::new(
                                center.x + dx as f32 * 10.0 * self.zoom,
                                center.y + dy as f32 * 10.0 * self.zoom,
                            ))
                            .scale(TetraVec2::new(self.zoom, self.zoom)),
                    )
                }
            }
        }
        self.world
            .avatar
            .draw(ctx, self.assets.clone(), center, self.zoom);
        self.redraw_sprites(ctx);
        if let GameMode::Examining(None) = self.mode {
            self.examination_text.draw(ctx);
        } else if let GameMode::Examining(Some(dir)) = self.mode {
            self.cursor.set_position(Position::new(
                center.x + dir.dx() as f32 * 10.0 * self.zoom,
                center.y + dir.dy() as f32 * 10.0 * self.zoom,
                AnchorX::Left,
                AnchorY::Top,
            ));
            self.cursor.set_scale(TetraVec2::new(self.zoom, self.zoom));
            self.cursor.positionate(ctx, window_size);
            self.cursor.draw(ctx);
        }
        for (i, text) in self.log.iter_mut().enumerate() {
            text.draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(
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

    fn on_resize(&mut self, ctx: &mut Context) {
        let window_size = window::get_size(ctx);
        for sprite in self.sprites.iter() {
            sprite.borrow_mut().positionate(ctx, window_size);
        }
        self.examination_text.positionate(ctx, window_size);
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
