use actions::{Action, ActionType};
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
        let action = ActionType::Dropping(0, dir);
        if action.is_possible(&game.world) {
            let length = action.length(&game.world);
            let finish = game.world.meta.current_tick + length;
            // TODO: probably return something like Transition::StartAction, not setting it directly
            game.world.player_mut().action = Some(Action::new(finish, action));
        } else {
            println!("You can't drop items here!");
        }
        game.mode = GameMode::Default;
        game.selected = None;
    }
    None
}
