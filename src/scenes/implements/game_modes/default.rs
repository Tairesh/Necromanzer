use actions::{Action, ActionType};
use geometry::direction::Direction;
use input;
use map::item::ItemType;
use scenes::game_mode::GameMode;
use scenes::game_scene::GameScene;
use scenes::implements::Game;
use scenes::transition::{SomeTransitions, Transition};
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

pub fn update(game: &mut Game, ctx: &mut Context) -> SomeTransitions {
    if input::is_key_pressed(ctx, Key::Escape) {
        return Some(vec![Transition::Push(GameScene::GameMenu)]);
    } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
        game.mode = GameMode::Examining;
    } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
        if game.world.player().wield.is_empty() {
            // TODO: log
            println!("You have nothing to drop!");
        } else {
            let action = ActionType::Dropping(0, Direction::Here);
            if action.is_possible(&game.world) {
                let length = action.length(&game.world);
                let finish = game.world.meta.current_tick + length;
                game.world.player_mut().action = Some(Action::new(finish, action));
            } else {
                println!("You can't put items here!");
            }
        }
    } else if input::is_key_pressed(ctx, Key::D)
        && input::is_key_modifier_down(ctx, KeyModifier::Shift)
    {
        if game.world.player().wield.is_empty() {
            println!("You have nothing to drop!");
        } else {
            game.mode = GameMode::Dropping;
        }
    } else if input::is_key_pressed(ctx, Key::W) && input::is_no_key_modifiers(ctx) {
        if !game.world.player().wield.is_empty() {
            // TODO: check limit of hands
            println!(
                "You are already wielding the {}",
                game.world.player().wield.last().unwrap().item_type.name()
            );
        } else {
            game.mode = GameMode::Wielding;
        }
        // } else if input::is_key_pressed(ctx, Key::C)
        //     && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        // {
        //     // game.log.clear();
    } else if input::is_key_pressed(ctx, Key::G) && input::is_no_key_modifiers(ctx) {
        if game
            .world
            .player()
            .wield
            .iter()
            .any(|i| matches!(i.item_type, ItemType::Shovel))
        {
            game.mode = GameMode::Digging;
        } else {
            println!("You can't dig without a shovel");
        }
    } else if input::is_key_pressed(ctx, Key::Num2)
        && input::is_key_modifier_down(ctx, KeyModifier::Shift)
    {
        return Some(vec![Transition::Push(GameScene::Empty)]);
    }
    let now = Instant::now();
    if let Some(dir) = input::get_direction_keys_down(ctx) {
        if now.duration_since(game.last_walk).as_millis() > 125
            || input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            game.last_walk = now;
            if dir.is_here() {
                let action = ActionType::SkippingTime;
                let finish = game.world.meta.current_tick + action.length(&game.world);
                game.world.player_mut().action = Some(Action::new(finish, action));
            } else {
                // TODO: move length calc and possibility checks to Action::new()
                let action = ActionType::Walking(dir);
                if action.is_possible(&game.world) {
                    let length = action.length(&game.world);
                    let finish = game.world.meta.current_tick + length;
                    game.world.player_mut().action = Some(Action::new(finish, action));
                }
            }
        }
    }
    None
}
