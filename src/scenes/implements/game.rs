use app::App;
use assets::game_data::GameData;
use assets::tileset::Tileset;
use colors::Colors;
use game::actions::Action;
use game::World;
use geometry::direction::{Direction, TwoDimDirection, DIR9};
use geometry::Vec2;
use scenes::game_modes::GameMode;
use scenes::scene_impl::SceneImpl;
use scenes::transition::SomeTransitions;
use sprites::label::Label;
use sprites::position::{Position, Vertical};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;
use std::time::Instant;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::Context;

pub struct Game {
    pub sprites: BunchOfSprites,
    pub world: World,
    pub game_data: Rc<GameData>,
    pub last_walk: Instant,
    pub mode: GameMode,
    pub cursor: Mesh,
    pub selected: Option<Direction>,
    pub current_time_label: Rc<RefCell<Label>>,
    pub tileset: Rc<Tileset>,
}

impl Game {
    pub fn new(world: World, app: &App, ctx: &mut Context) -> Self {
        let name_label = Rc::new(RefCell::new(Label::new(
            world.player().character.name.as_str(),
            app.assets.fonts.header2.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(50.0, 1.0),
        )));
        let current_time_label = Rc::new(RefCell::new(Label::new(
            format!("{}", world.meta.current_tick),
            app.assets.fonts.default2.clone(),
            Colors::WHITE_SMOKE,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 5.0 }),
        )));
        Self {
            sprites: vec![name_label, current_time_label.clone()],
            current_time_label,
            world,
            game_data: app.assets.game_data.clone(),
            last_walk: Instant::now(),
            mode: GameMode::Default,
            cursor: Mesh::rectangle(
                ctx,
                ShapeStyle::Stroke(1.0),
                Rectangle::new(
                    0.0,
                    0.0,
                    app.assets.tileset.tile_size as f32,
                    app.assets.tileset.tile_size as f32,
                ),
            )
            .unwrap(),
            selected: None,
            tileset: app.assets.tileset.clone(),
        }
    }

    pub fn select(&mut self, dir: Direction) {
        if self.selected.is_none() {
            self.selected = Some(dir);
            if let Ok(dir) = TwoDimDirection::try_from(dir) {
                self.world.player_mut().vision = dir;
            }
        }
    }

    pub fn call_action(&mut self, action: Result<Action, String>) {
        match action {
            Ok(action) => {
                self.world.player_mut().action = Some(action);
            }
            Err(msg) => {
                self.world.log(msg, Colors::LIGHT_YELLOW);
            }
        }
    }
}

impl SceneImpl for Game {
    fn update(&mut self, ctx: &mut Context) -> SomeTransitions {
        let game_mode = self.mode;
        if let Some(t) = game_mode.update(self, ctx) {
            return Some(t);
        }

        if let Some(action) = self.world.player().action {
            let starting_tick = self.world.meta.current_tick;
            self.world.tick();
            let delta = self.world.meta.current_tick - starting_tick;
            if delta > 20 && self.world.player().action.is_none() {
                self.world.log(
                    format!("It takes a long time to {}.", action.name(&self.world)),
                    Colors::LIGHT_GRAY,
                );
            }
            self.current_time_label.borrow_mut().update(
                format!("{}", self.world.meta.current_tick),
                ctx,
                tetra::window::get_size(ctx),
            )
        }
        None
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        tetra::graphics::clear(ctx, Colors::BLACK);
        let window_size = tetra::window::get_size(ctx);
        let zoom = self.world.game_view.zoom.as_view();
        let scale = self.world.game_view.zoom.as_scale();
        let window_size_in_tiles = (
            (window_size.0 as f32 / (self.tileset.tile_size as f32 * zoom)).ceil() as i32,
            (window_size.1 as f32 / (self.tileset.tile_size as f32 * zoom as f32)).ceil() as i32,
        );
        let center = Vec2::new(
            window_size.0 as f32 / 2.0 - 5.0 * zoom,
            window_size.1 as f32 / 2.0 - 5.0 * zoom,
        );
        let center_tile = self.world.player().pos;
        let left_top = center_tile + (-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
        let right_bottom = center_tile + (window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
        for (pos, tile) in self.world.tiles_between(left_top, right_bottom).into_iter() {
            let dx = pos.x - center_tile.x;
            let dy = pos.y - center_tile.y;
            let region = tile.terrain.region(&self.tileset);
            let params = DrawParams::new()
                .position(Vec2::new(
                    center.x + dx as f32 * self.tileset.tile_size as f32 * zoom,
                    center.y + dy as f32 * self.tileset.tile_size as f32 * zoom,
                ))
                .scale(scale);
            self.tileset
                .texture
                .draw_region(ctx, region, params.clone());
            if let Some(item) = tile.top_item() {
                self.tileset.texture.draw_region(
                    ctx,
                    item.item_type.region(&self.tileset),
                    params.clone(),
                );
                if tile.items.len() > 1 {
                    self.tileset
                        .texture
                        .draw_region(ctx, self.tileset.highlight, params);
                }
            }
        }
        for i in self.world.loaded_units.iter().copied() {
            let unit = self.world.units.get(i).unwrap();
            let dx = unit.pos.x - center_tile.x;
            let dy = unit.pos.y - center_tile.y;
            let position = Vec2::new(
                center.x + dx as f32 * self.tileset.tile_size as f32 * zoom,
                center.y + dy as f32 * self.tileset.tile_size as f32 * zoom,
            );
            unit.draw(ctx, &self.tileset, position, zoom, true);
        }
        // if self.world.player().action.is_some() {
        //     self.draw_action_loader(ctx, center);
        // } else {
        //     self.action_text = None;
        // }
        if self.mode.draw_cursors() {
            for dir in DIR9 {
                let pos = self.world.player().pos + dir;
                if self.mode.draw_cursor_here(self.world.load_tile(pos)) {
                    let delta = dir.as_vec() * self.tileset.tile_size as f32 * zoom;
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
                Vec2::new(dir.dx() as f32, dir.dy() as f32) * self.tileset.tile_size as f32 * zoom;
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .scale(scale)
                    .position(center + delta)
                    .color(Colors::LIGHT_YELLOW.with_alpha(0.7)),
            )
        }
        for (i, msg) in self
            .world
            .log
            .as_mut()
            .unwrap()
            .texts
            .iter_mut()
            .enumerate()
        {
            msg.draw(
                Vec2::new(10.0, window_size.1 as f32 - 20.0 - 20.0 * i as f32),
                ctx,
            );
        }
    }

    fn after_draw(&mut self, ctx: &mut Context) {
        // UI
        self.world
            .player()
            .draw(ctx, &self.tileset, Vec2::new(5.0, 5.0), 3.0, false);
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.world.save();
    }
}
