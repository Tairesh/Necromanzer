use game::actions::ActionType;
use geometry::direction::Direction;
use input;
use scenes::game_modes::implements::digging::Digging;
use scenes::game_modes::implements::dropping::Dropping;
use scenes::game_modes::implements::examining::Examining;
use scenes::game_modes::implements::observing::Observing;
use scenes::game_modes::implements::wielding::Wielding;
use scenes::game_modes::{GameModeImpl, SomeResults, UpdateResult};
use scenes::scene::Scene;
use scenes::transition::Transition;
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::math::Vec2;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
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
    fn update(&mut self, ctx: &mut Context) -> SomeResults {
        if input::is_mouse_scrolled_down(ctx) {
            UpdateResult::ZoomOut.into()
        } else if input::is_mouse_scrolled_up(ctx) {
            UpdateResult::ZoomIn.into()
        } else if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::SceneTransit(vec![Transition::Push(Scene::GameMenu)]).into()
        } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
            UpdateResult::Push(Examining::new().into()).into()
        } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
            UpdateResult::TryStartAction(ActionType::Dropping(0, Direction::Here)).into()
        } else if input::is_key_pressed(ctx, Key::D)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            UpdateResult::Push(Dropping::new().into()).into()
        } else if input::is_key_pressed(ctx, Key::W) && input::is_no_key_modifiers(ctx) {
            UpdateResult::Push(Wielding::new().into()).into()
        } else if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            UpdateResult::ClearLog.into()
        } else if input::is_key_pressed(ctx, Key::G) && input::is_no_key_modifiers(ctx) {
            UpdateResult::Push(Digging::new().into()).into()
        } else if input::is_key_pressed(ctx, Key::X) && input::is_no_key_modifiers(ctx) {
            UpdateResult::Push(
                Observing {
                    cursor: Vec2::zero(),
                }
                .into(),
            )
            .into()
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_walk).as_millis() > 125
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                if dir.is_here() {
                    UpdateResult::TryStartAction(ActionType::SkippingTime).into()
                } else {
                    Some(vec![
                        UpdateResult::TryRotate(dir),
                        UpdateResult::TryStartAction(ActionType::Walking(dir)),
                    ])
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
