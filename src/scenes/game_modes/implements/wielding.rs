use colors::Colors;
use game::actions::ActionType;
use game::map::item::ItemView;
use game::World;
use geometry::direction::{Direction, DIR9};
use geometry::point::Point;
use input;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

pub struct Wielding {
    selected: Option<Direction>,
}

impl Wielding {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Wielding {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Wielding {
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
                        .map(|t| !t.items.is_empty())
                        .unwrap_or(false)
                })
                .map(|d| (d.into(), Colors::WHITE_SMOKE))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        // TODO: hands counting
        if world.player().wield.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "You already have {} in hands",
                world.player().wield.last().unwrap().name()
            ))
        }
    }

    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeResults {
        if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::Pop.into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            self.selected = Some(dir);
            game.try_rotate_player(dir);
            None
        } else {
            self.selected.map(|dir| {
                game.try_start_action(ActionType::Wielding(dir));
                vec![UpdateResult::Pop]
            })
        }
    }
}
