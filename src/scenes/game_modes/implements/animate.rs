use colors::Colors;
use game::actions::ActionType;
use game::map::item::Item;
use game::World;
use geometry::direction::{Direction, DIR9};
use geometry::point::Point;
use input;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

pub struct Animate {
    selected: Option<Direction>,
}

impl Animate {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Animate {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Animate {
    fn cursors(&self, world: &World) -> Vec<(Point, Color)> {
        if let Some(selected) = self.selected {
            vec![(selected.into(), Colors::LIME)]
        } else {
            DIR9.iter()
                .copied()
                .filter(|d| {
                    let pos = world.player().pos + d;
                    world
                        .get_tile(pos)
                        .map(|t| t.items.iter().any(|i| matches!(i, Item::Corpse(..))))
                        .unwrap_or(false)
                })
                .map(|d| (d.into(), Colors::LIGHT_YELLOW))
                .collect()
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.modes.pop();
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
        } else if let Some(dir) = self.selected {
            game.try_start_action(ActionType::Animate(dir));
            game.modes.pop();
        }
        None
    }
}
