use colors::Colors;
use game::World;
use geometry::direction::Direction;
use geometry::point::Point;
use input;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::scene::Scene::BodyView;
use scenes::transition::{SomeTransitions, Transition};
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

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

    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            let mut world = game.world.borrow_mut();
            let pos = world.player().pos + dir;
            let tile = world.load_tile(pos);
            let unit_id = tile.units.iter().next();
            if let Some(unit_id) = unit_id {
                game.modes.pop();
                return Some(vec![Transition::Push(BodyView(*unit_id))]);
            }
            drop(world);
            game.examine(dir);
            game.modes.pop();
        }
        None
    }
}
