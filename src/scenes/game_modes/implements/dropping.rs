use super::super::GameMode;
use game::actions::{Action, ActionType};
use input;
use map::tile::Tile;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::input::Key;
use tetra::Context;

pub struct Dropping {}

impl From<Dropping> for GameMode {
    fn from(_: Dropping) -> Self {
        GameMode::Dropping
    }
}

impl GameModeImpl for Dropping {
    fn draw_cursors(&self) -> bool {
        true
    }

    fn draw_cursor_here(&self, tile: &Tile) -> bool {
        tile.terrain.is_walkable()
    }

    fn update(&self, game: &mut Game, ctx: &mut Context) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.mode = GameMode::Default;
            game.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            game.select(dir);
        } else if let Some(dir) = game.selected {
            game.call_action(Action::new(ActionType::Dropping(0, dir), &game.world));
            game.mode = GameMode::Default;
            game.selected = None;
        }
        None
    }
}
