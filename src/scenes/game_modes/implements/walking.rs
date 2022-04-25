use colors::Colors;
use game::actions::ActionType;
use geometry::direction::Direction;
use input;
use map::item::ItemView;
use scenes::game_modes::implements::*;
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
        } else if input::is_key_with_mod_pressed(ctx, Key::E) {
            UpdateResult::Push(Examining::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, Key::D) {
            game.try_start_action(ActionType::Dropping(0, Direction::Here));
            None
        } else if input::is_key_with_mod_pressed(ctx, (Key::D, KeyModifier::Shift)) {
            UpdateResult::Push(Dropping::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, Key::W) {
            UpdateResult::Push(Wielding::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, (Key::C, KeyModifier::Shift)) {
            game.log.clear();
            None
        } else if input::is_key_with_mod_pressed(ctx, Key::G) {
            UpdateResult::Push(Digging::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, Key::X) {
            UpdateResult::Push(Observing::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, (Key::R, KeyModifier::Shift)) {
            UpdateResult::Push(Reading::new().into()).into()
        } else if input::is_key_with_mod_pressed(ctx, (Key::Num2, KeyModifier::Shift)) {
            UpdateResult::SceneTransit(vec![Transition::Push(Scene::BodyView(0))]).into()
        } else if input::is_key_with_mod_pressed(ctx, Key::I) {
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
        } else if input::is_key_with_mod_pressed(ctx, Key::A) {
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
