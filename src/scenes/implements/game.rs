use app::App;
use assets::game_data::GameData;
use assets::Assets;
use colors::Colors;
use game::actions::{Action, ActionResult, ActionType};
use game::{Log, World};
use geometry::direction::{Direction, TwoDimDirection};
use geometry::point::Point;
use geometry::Vec2;
use map::item::ItemView;
use map::terrain::TerrainView;
use scenes::game_modes::implements::Walking;
use scenes::game_modes::GameModeImpl;
use scenes::game_modes::{GameMode, UpdateResult};
use scenes::scene_impl::SceneImpl;
use scenes::transition::SomeTransitions;
use settings::game::GameSettings;
use sprites::label::Label;
use sprites::position::{Position, Vertical};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::Context;

pub struct Game {
    pub sprites: BunchOfSprites,
    pub world: Rc<RefCell<World>>,
    pub game_data: Rc<GameData>,
    pub modes: Vec<Rc<RefCell<GameMode>>>,
    pub cursor: Mesh,
    pub current_time_label: Rc<RefCell<Label>>,
    pub log: Log,
    pub shift_of_view: Point,
    pub settings: Rc<RefCell<GameSettings>>,
    pub assets: Rc<Assets>,
    pub window_size: (i32, i32),
    pub fov: HashSet<Point>,
}

impl Game {
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let world = app.clone_world();
        let name_label = Rc::new(RefCell::new(Label::new(
            world.borrow().player().character.name.as_str(),
            app.assets.fonts.header2.clone(),
            Colors::WHITE_SMOKE,
            Position::by_left_top(50.0, 1.0),
        )));
        let current_time_label = Rc::new(RefCell::new(Label::new(
            format!("{}", world.borrow().meta.current_tick),
            app.assets.fonts.default2.clone(),
            Colors::WHITE_SMOKE,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 5.0 }),
        )));
        let fov = world.borrow().fov.clone();
        Self {
            sprites: vec![name_label, current_time_label.clone()],
            game_data: app.assets.game_data.clone(),
            modes: vec![Rc::new(RefCell::new(Walking::new().into()))],
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
            log: Log::new(app.assets.fonts.default.font.clone()),
            shift_of_view: Point::zero(),
            settings: app.settings.clone(),
            assets: app.assets.clone(),
            window_size: app.window_size,
            current_time_label,
            world,
            fov,
        }
    }

    pub fn current_mode(&self) -> Rc<RefCell<GameMode>> {
        self.modes.last().unwrap().clone()
    }

    pub fn push_mode(&mut self, mode: GameMode) {
        match mode.can_push(&self.world.borrow()) {
            Ok(..) => self.modes.push(Rc::new(RefCell::new(mode))),
            Err(s) => {
                self.log.log(s, Colors::LIGHT_CORAL);
            }
        }
    }

    pub fn try_rotate_player(&mut self, dir: Direction) {
        if let Ok(dir) = TwoDimDirection::try_from(dir) {
            self.world.borrow_mut().player_mut().vision = dir;
        }
    }

    pub fn examine(&mut self, dir: Direction) {
        let pos = self.world.borrow().player().pos + dir;
        self.log
            .log(self.world.borrow().this_is(pos, false), Colors::WHITE_SMOKE);
    }

    pub fn try_start_action(&mut self, typ: ActionType) {
        let action = Action::new(0, typ, &self.world.borrow());
        match action {
            Ok(action) => {
                self.world.borrow_mut().player_mut().action = Some(action);
            }
            Err(msg) => {
                if !self.log.same_message(&msg) {
                    self.log.log(msg, Colors::LIGHT_CORAL);
                }
            }
        }
    }

    pub fn mode_update(&mut self, ctx: &mut Context) -> SomeTransitions {
        if let Some(updates) = self.current_mode().borrow_mut().update(ctx, self) {
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
                    UpdateResult::SceneTransit(t) => {
                        return Some(t);
                    }
                }
            }
        }

        None
    }

    pub fn tile_size(&self) -> f32 {
        self.assets.tileset.tile_size as f32 * self.world.borrow().game_view.zoom.as_view()
    }

    fn make_world_tick(&mut self, ctx: &mut Context) {
        let mut world = self.world.borrow_mut();
        for action in world.tick() {
            match action {
                ActionResult::LogMessage(message) => {
                    self.log.log(message, Colors::WHITE_SMOKE);
                }
                ActionResult::ColoredLogMessage(message, color) => {
                    self.log.log(message, color);
                }
            }
        }

        self.current_time_label.borrow_mut().update(
            format!("{}", world.meta.current_tick),
            ctx,
            self.window_size,
        );
        if world.fov_dirty {
            self.fov = world.fov.clone();
            world.fov_dirty = false;
        }
    }
}

impl SceneImpl for Game {
    fn update(&mut self, ctx: &mut Context) -> SomeTransitions {
        if self.world.borrow().player().action.is_some() {
            self.make_world_tick(ctx);

            None
        } else {
            self.mode_update(ctx)
        }
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        tetra::graphics::clear(ctx, Colors::BLACK);
        let width = self.window_size.0 as f32;
        let height = self.window_size.1 as f32;
        let zoom = self.world.borrow().game_view.zoom;
        let scale = zoom.as_scale();
        let zoom = zoom.as_view();
        let tile_size = self.tile_size();
        let window_size_in_tiles = (
            (width as f32 / tile_size).ceil() as i32,
            (height as f32 / tile_size).ceil() as i32,
        );
        let center = Vec2::new(
            width / 2.0 - tile_size / 2.0,
            height / 2.0 - tile_size / 2.0,
        );
        let center_tile = self.world.borrow().player().pos + self.shift_of_view;
        let left_top = center_tile + (-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
        let right_bottom = center_tile + (window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
        for (pos, tile) in self
            .world
            .borrow_mut()
            .tiles_between(left_top, right_bottom)
            .into_iter()
        {
            if !self.fov.contains(&pos.into()) {
                continue; // TODO: TileView struct for remembering tiles and optimizing drawing
            }
            let dx = pos.x - center_tile.x;
            let dy = pos.y - center_tile.y;
            let params = DrawParams::new()
                .position(Vec2::new(
                    center.x + dx as f32 * tile_size,
                    center.y + dy as f32 * tile_size,
                ))
                .scale(scale);
            self.assets
                .tileset
                .draw_region(ctx, tile.terrain.looks_like(), params.clone());
            if let Some(item) = tile.top_item() {
                self.assets
                    .tileset
                    .draw_region(ctx, item.looks_like(), params.clone());
                if tile.items.len() > 1 {
                    self.assets.tileset.draw_region(ctx, "highlight", params);
                }
            }
        }
        for i in self.world.borrow().loaded_units.iter().copied() {
            let world = self.world.borrow();
            let unit = world.units.get(i).unwrap();
            if !self.fov.contains(&unit.pos.into()) {
                continue;
            }
            let dx = unit.pos.x - center_tile.x;
            let dy = unit.pos.y - center_tile.y;
            let position = Vec2::new(
                center.x + dx as f32 * tile_size,
                center.y + dy as f32 * tile_size,
            );
            unit.draw(ctx, &self.assets.tileset, position, zoom, true);
        }
        // if self.world.borrow().player().action.is_some() {
        //     self.draw_action_loader(ctx, center);
        // } else {
        //     self.action_text = None;
        // }
        for (delta, color) in self.current_mode().borrow().cursors(&self.world.borrow()) {
            let delta = delta * tile_size;
            self.cursor.draw(
                ctx,
                DrawParams::new()
                    .position(center + delta)
                    .scale(scale)
                    .color(color.with_alpha(0.7)),
            );
        }

        for (i, msg) in self.log.texts.iter_mut().enumerate() {
            msg.draw(Vec2::new(10.0, height - 20.0 * (i + 1) as f32), ctx);
        }
    }

    fn after_draw(&mut self, ctx: &mut Context) {
        // UI
        self.world.borrow().player().draw(
            ctx,
            &self.assets.tileset,
            Vec2::new(5.0, 5.0),
            3.0,
            false,
        );
        self.current_mode().borrow_mut().draw(ctx, self);
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.world.borrow_mut().save();
    }
}
