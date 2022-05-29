use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

use crate::{
    colors::Colors,
    game::World,
    geometry::{Direction, Point},
    input,
};

use super::super::{
    super::{implements::GameScene, Scene, SomeTransitions, Transition},
    GameModeImpl,
};

pub struct Examining {
    selected: Option<Direction>,
}

impl Examining {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Examining {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Examining {
    fn cursors(&self, _world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            vec![]
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            let world = game.world.borrow();
            let pos = world.player().pos + dir;
            let unit_id = world.map().get_tile(pos).units.iter().copied().next();
            if let Some(unit_id) = unit_id {
                game.modes.pop();
                return Some(vec![Transition::Push(Scene::BodyView(unit_id))]);
            }
            drop(world);
            game.examine(dir);
            game.modes.pop();
        }
        None
    }
}
