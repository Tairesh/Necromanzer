use action::ActionType;
use input;
use scenes::game::examining::Examining;
use scenes::game::{GameModeTrait, UpdateResult};
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::Context;

pub(crate) struct Walking {
    pub last_walk: Instant,
}

impl Walking {
    pub fn new() -> Self {
        Self {
            last_walk: Instant::now(),
        }
    }
}

impl GameModeTrait for Walking {
    fn update(&mut self, ctx: &mut Context) -> UpdateResult {
        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(self.last_walk).as_millis() > 75
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                return if dir.is_here() {
                    UpdateResult::SetAvatarAction(ActionType::SkippingTime)
                } else {
                    UpdateResult::SetAvatarAction(ActionType::Walking(dir))
                };
            }
        }
        if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            UpdateResult::ClearLog
        } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
            UpdateResult::Drop
        } else if input::is_key_pressed(ctx, Key::Escape) {
            UpdateResult::OpenMenu
        } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
            UpdateResult::SwitchGameMode(Examining::new(ctx).into())
        } else {
            UpdateResult::DoNothing
        }
    }
}
