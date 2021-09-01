use tetra::input::{self, Key, KeyModifier};
use tetra::Context;

pub fn is_no_key_modifiers(ctx: &Context) -> bool {
    input::is_key_modifier_up(ctx, KeyModifier::Shift)
        && input::is_key_modifier_up(ctx, KeyModifier::Alt)
        && input::is_key_modifier_up(ctx, KeyModifier::Ctrl)
}

pub fn get_direction_keys_down(ctx: &Context) -> (i32, i32) {
    let (mut moving_x, mut moving_y) = (0, 0);
    if input::is_key_down(ctx, Key::Up)
        || input::is_key_down(ctx, Key::NumPad8)
        || (input::is_key_down(ctx, Key::Num8)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_y -= 1;
    }
    if input::is_key_down(ctx, Key::Down)
        || input::is_key_down(ctx, Key::NumPad2)
        || (input::is_key_down(ctx, Key::Num2)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_y += 1;
    }
    if input::is_key_down(ctx, Key::Left)
        || input::is_key_down(ctx, Key::NumPad4)
        || (input::is_key_down(ctx, Key::Num4)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x -= 1;
    }
    if input::is_key_down(ctx, Key::Right)
        || input::is_key_down(ctx, Key::NumPad6)
        || (input::is_key_down(ctx, Key::Num6)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x += 1;
    }
    if input::is_key_down(ctx, Key::NumPad9)
        || (input::is_key_down(ctx, Key::Num9)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x += 1;
        moving_y -= 1;
    }
    if input::is_key_down(ctx, Key::NumPad7)
        || (input::is_key_down(ctx, Key::Num7)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x -= 1;
        moving_y -= 1;
    }
    if input::is_key_down(ctx, Key::NumPad1)
        || (input::is_key_down(ctx, Key::Num1)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x -= 1;
        moving_y += 1;
    }
    if input::is_key_down(ctx, Key::NumPad3)
        || (input::is_key_down(ctx, Key::Num3)
            && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    {
        moving_x += 1;
        moving_y += 1;
    }

    (moving_x, moving_y)
}

pub fn get_skipping_time_key_down(ctx: &Context) -> bool {
    input::is_key_down(ctx, Key::NumPad5)
        || input::is_key_down(ctx, Key::Num5)
        || input::is_key_down(ctx, Key::RightBracket)
}
