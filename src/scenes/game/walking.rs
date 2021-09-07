use action::ActionType;
use direction::Direction;
use input;
use scenes::game::dropping::Dropping;
use scenes::game::examining::Examining;
use scenes::game::{GameModeTrait, UpdateResult};
use std::time::Instant;
use tetra::input::{Key, KeyModifier};
use tetra::Context;
use world::World;

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
    fn update(&mut self, ctx: &mut Context, world: &mut World) -> Vec<UpdateResult> {
        let now = Instant::now();
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if now.duration_since(self.last_walk).as_millis() > 75
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                return vec![if dir.is_here() {
                    UpdateResult::SetAvatarAction(ActionType::SkippingTime)
                } else {
                    UpdateResult::SetAvatarAction(ActionType::Walking(dir))
                }];
            }
        }
        if input::is_key_pressed(ctx, Key::C)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            vec![UpdateResult::ClearLog]
        } else if input::is_key_pressed(ctx, Key::D) && input::is_no_key_modifiers(ctx) {
            vec![UpdateResult::Drop(Direction::Here)]
        } else if input::is_key_pressed(ctx, Key::D)
            && input::is_key_modifier_down(ctx, KeyModifier::Shift)
        {
            if world.avatar.wield.is_empty() {
                vec![UpdateResult::AddLogMessage(
                    "You have nothing to drop!".to_string(),
                )]
            } else {
                vec![UpdateResult::SwitchGameMode(Dropping::new(ctx).into())]
            }
        } else if input::is_key_pressed(ctx, Key::Escape) {
            vec![UpdateResult::OpenMenu]
        } else if input::is_key_pressed(ctx, Key::E) && input::is_no_key_modifiers(ctx) {
            vec![UpdateResult::SwitchGameMode(Examining::new(ctx).into())]
        } else {
            vec![]
        }
    }
}
