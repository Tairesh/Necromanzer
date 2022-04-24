use colors::Colors;
use game::actions::ActionType;
use geometry::direction::Direction;
use input;
use map::item::ItemView;
use scenes::game_modes::implements::animate::Animate;
use scenes::game_modes::implements::digging::Digging;
use scenes::game_modes::implements::dropping::Dropping;
use scenes::game_modes::implements::examining::Examining;
use scenes::game_modes::implements::observing::Observing;
use scenes::game_modes::implements::reading::Reading;
use scenes::game_modes::implements::wielding::Wielding;
use scenes::game_modes::{GameModeImpl, SomeResults, UpdateResult};
use scenes::implements::Game;
use scenes::scene::Scene;
use scenes::transition::Transition;
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

pub struct Walking {
    last_walk: Instant,
}

impl Walking {
    pub fn new() -> Self {
        Self {
            last_walk: Instant::now(),
        }
    }
}

impl Default for Walking {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Walking {
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeResults {
        if input::is_mouse_scrolled_down(ctx) {
            game.world.borrow_mut().game_view.zoom.dec();
            None
        } else if input::is_mouse_scrolled_up(ctx) {
            game.world.borrow_mut().game_view.zoom.inc();
            None
        } else if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::SceneTransit(vec![Transition::Push(Scene::GameMenu)]).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::E, None) {
            UpdateResult::Push(Examining::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::D, None) {
            game.try_start_action(ActionType::Dropping(0, Direction::Here));
            None
        } else if input::is_pressed_key_with_mod(ctx, Key::D, Some(KeyModifier::Shift)) {
            UpdateResult::Push(Dropping::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::W, None) {
            UpdateResult::Push(Wielding::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::C, Some(KeyModifier::Shift)) {
            game.log.clear();
            None
        } else if input::is_pressed_key_with_mod(ctx, Key::G, None) {
            UpdateResult::Push(Digging::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::X, None) {
            UpdateResult::Push(Observing::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::R, Some(KeyModifier::Shift)) {
            UpdateResult::Push(Reading::new().into()).into()
        } else if input::is_pressed_key_with_mod(ctx, Key::Num2, Some(KeyModifier::Shift)) {
            // TODO: body view game scene (gamemodes should not use a lot of sprites)
            None
        } else if input::is_pressed_key_with_mod(ctx, Key::I, None) {
            // TODO: inventory game scene
            let items: Vec<String> = game
                .world
                .borrow()
                .player()
                .body
                .wear
                .iter()
                .map(|i| i.name())
                .collect();
            game.log.log(
                format!("You wear: {}", items.join(", ")),
                Colors::WHITE_SMOKE,
            );
            None
        } else if input::is_pressed_key_with_mod(ctx, Key::A, None) {
            UpdateResult::Push(Animate::new().into()).into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_walk).subsec_millis()
                > game.settings.borrow().repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                if dir.is_here() {
                    game.try_start_action(ActionType::SkippingTime);
                } else {
                    game.try_rotate_player(dir);
                    game.try_start_action(ActionType::Walking(dir));
                }
            }
            None
        } else {
            None
        }
    }
}
