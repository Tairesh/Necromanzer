use game::actions::{Action, ActionType};
use input;
use scenes::game_mode::GameMode;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::input::Key;
use tetra::Context;

pub fn update(game: &mut Game, ctx: &mut Context) -> SomeTransitions {
    if input::is_key_pressed(ctx, Key::Escape) {
        game.mode = GameMode::Default;
        game.selected = None;
    }
    if let Some(dir) = input::get_direction_keys_down(ctx) {
        game.select(dir);
    } else if let Some(dir) = game.selected {
        game.call_action(Action::new(ActionType::Digging(dir), &game.world));
        game.mode = GameMode::Default;
        game.selected = None;
    }

    None
}
