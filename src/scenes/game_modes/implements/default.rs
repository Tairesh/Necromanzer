use super::super::GameMode;
use colors::Colors;
use game::actions::{Action, ActionType};
use geometry::direction::Direction;
use input;
use map::item::ItemType;
use map::tile::Tile;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::scene::Scene;
use scenes::transition::{SomeTransitions, Transition};
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

pub struct Default {}

impl From<Default> for GameMode {
    fn from(_: Default) -> Self {
        GameMode::Default
    }
}

impl GameModeImpl for Default {
    fn draw_cursors(&self) -> bool {
        false
    }

    fn draw_cursor_here(&self, _tile: &Tile) -> bool {
        false
    }

    fn update(&self, game: &mut Game, ctx: &mut Context) -> SomeTransitions {
        if input::is_mouse_scrolled_down(ctx) {
            game.world.game_view.zoom.dec();
        }
        if input::is_mouse_scrolled_up(ctx) {
            game.world.game_view.zoom.inc();
        }
        if input::is_key_pressed(ctx, Key::Escape) {
            return Some(vec![Transition::Push(Scene::GameMenu)]);
        } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
            game.mode = GameMode::Examining;
        } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
            game.call_action(Action::new(
                ActionType::Dropping(0, Direction::Here),
                &game.world,
            ));
        } else if input::is_key_pressed(ctx, Key::D)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            if game.world.player().wield.is_empty() {
                game.world.log("You have nothing to drop!", Colors::ORANGE);
            } else {
                game.mode = GameMode::Dropping;
            }
        } else if input::is_key_pressed(ctx, Key::W) && input::is_no_key_modifiers(ctx) {
            if !game.world.player().wield.is_empty() {
                // TODO: check limit of hands
                game.world.log(
                    format!(
                        "You are already wielding the {}",
                        game.world.player().wield.last().unwrap().item_type.name()
                    ),
                    Colors::ORANGE,
                );
            } else {
                game.mode = GameMode::Wielding;
            }
        } else if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            game.world.log.as_mut().unwrap().clear();
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
                game.world
                    .log("You can't dig without a shovel", Colors::ORANGE);
            }
        } else if input::is_key_pressed(ctx, Key::Num2)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            return Some(vec![Transition::Push(Scene::Empty)]);
        }
        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(game.last_walk).as_millis() > 125
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                game.last_walk = now;
                game.call_action(Action::new(
                    if dir.is_here() {
                        ActionType::SkippingTime
                    } else {
                        ActionType::Walking(dir)
                    },
                    &game.world,
                ));
            }
        }
        None
    }
}
