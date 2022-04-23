use colors::Colors;
use game::actions::ActionType;
use game::World;
use geometry::direction::{Direction, DIR9};
use geometry::point::Point;
use input;
use map::item::Item;
use map::terrain::TerrainInteract;
use scenes::game_modes::update_result::UpdateResult;
use scenes::game_modes::{GameModeImpl, SomeResults};
use scenes::implements::Game;
use tetra::graphics::Color;
use tetra::input::Key;
use tetra::Context;

pub struct Digging {
    selected: Option<Direction>,
}

impl Digging {
    pub fn new() -> Self {
        Self { selected: None }
    }
}

impl Default for Digging {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Digging {
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
                        .map(|t| t.terrain.is_diggable())
                        .unwrap_or(false)
                })
                .map(|d| (d.into(), Colors::LIGHT_YELLOW))
                .collect()
        }
    }

    fn can_push(&self, world: &World) -> Result<(), String> {
        // TODO: use item tag/quality
        if world
            .player()
            .wield
            .iter()
            .any(|i| matches!(i, Item::Shovel(..)))
        {
            Ok(())
        } else {
            Err("You can't dig without a shovel".to_string())
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
                game.try_start_action(ActionType::Digging(dir));
                vec![UpdateResult::Pop]
            })
        }
    }
}
