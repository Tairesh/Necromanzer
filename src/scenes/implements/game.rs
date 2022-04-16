use app::App;
use assets::game_data::GameData;
use assets::tileset::Tileset;
use colors::Colors;
use game::actions::{Action, ActionResult};
use game::{Log, World};
use geometry::direction::TwoDimDirection;
use geometry::Vec2;
use scenes::game_modes::implements::walking::Walking;
use scenes::game_modes::GameModeImpl;
use scenes::game_modes::{GameMode, UpdateResult};
use scenes::scene_impl::SceneImpl;
use scenes::transition::SomeTransitions;
use sprites::label::Label;
use sprites::position::{Position, Vertical};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::Context;

pub struct Game {
    pub sprites: BunchOfSprites,
    pub world: World,
    pub game_data: Rc<GameData>,
    pub modes: Vec<GameMode>,
    pub cursor: Mesh,
    pub current_time_label: Rc<RefCell<Label>>,
    pub tileset: Rc<Tileset>,
    pub log: Log,
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
            modes: vec![Walking::new().into()],
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
            tileset: app.assets.tileset.clone(),
            log: Log::new(app.assets.fonts.default.font.clone()),
        }
    }

    pub fn current_mode(&self) -> &GameMode {
        self.modes.last().unwrap()
    }

    pub fn current_mode_mut(&mut self) -> &mut GameMode {
        self.modes.last_mut().unwrap()
    }

    pub fn push_mode(&mut self, mode: GameMode) {
        match mode.can_push(self) {
            Ok(..) => self.modes.push(mode),
            Err(s) => {
                self.log.log(s, Colors::ORANGE);
            }
        }
    }

    pub fn mode_update(&mut self, ctx: &mut Context) -> SomeTransitions {
        if let Some(updates) = self.current_mode_mut().update(ctx) {
            for update in updates {
                match update {
                    UpdateResult::Push(mode) => {
                        self.push_mode(mode);
                    }
                    UpdateResult::Replace(mode) => {
                        self.modes.pop();
                        self.push_mode(mode);
                    }
                    UpdateResult::Pop => {
                        self.modes.pop();
                    }
                    UpdateResult::ZoomIn => {
                        self.world.game_view.zoom.inc();
                    }
                    UpdateResult::ZoomOut => {
                        self.world.game_view.zoom.dec();
                    }
                    UpdateResult::TryStartAction(typ) => match Action::new(typ, &self.world) {
                        Ok(action) => {
                            self.world.player_mut().action = Some(action);
                        }
                        Err(msg) => {
                            self.log.log(msg, Colors::ORANGE_RED);
                        }
                    },
                    UpdateResult::ClearLog => {
                        self.log.clear();
                    }
                    UpdateResult::SceneTransit(t) => {
                        return Some(t);
                    }
                    UpdateResult::Examine(dir) => {
                        let pos = self.world.player().pos + dir;
                        let tile = self.world.load_tile(pos);
                        let mut this_is = tile.terrain.this_is();
                        if !tile.items.is_empty() {
                            let items: Vec<String> = tile
                                .items
                                .iter()
                                .map(|item| item.item_type.name())
                                .collect();
                            this_is += " Here you see: ";
                            this_is += items.join(", ").as_str();
                        }
                        self.log.log(this_is, Colors::WHITE_SMOKE);
                    }
                    UpdateResult::TryRotate(dir) => {
                        if let Ok(dir) = TwoDimDirection::try_from(dir) {
                            self.world.player_mut().vision = dir;
                        }
                    }
                }
            }
        }

        None
    }
}

impl SceneImpl for Game {
    fn update(&mut self, ctx: &mut Context) -> SomeTransitions {
        if self.world.player().action.is_some() {
            for action in self.world.tick() {
                match action {
                    ActionResult::LogMessage(s) => {
                        self.log.log(s, Colors::WHITE_SMOKE);
                    }
                }
            }
            self.current_time_label.borrow_mut().update(
                format!("{}", self.world.meta.current_tick),
                ctx,
                tetra::window::get_size(ctx),
            );

            None
        } else {
            self.mode_update(ctx)
        }
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
        for (delta, color) in self.current_mode().cursors(self) {
            let delta = delta * self.tileset.tile_size as f32 * zoom;
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .position(center + delta)
                    .scale(scale)
                    .color(color.with_alpha(0.7)),
            );
        }

        for (i, msg) in self.log.texts.iter_mut().enumerate() {
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
