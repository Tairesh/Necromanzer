use direction::Direction;
use tetra::input::{self, Key, KeyModifier};
use tetra::Context;

pub fn is_key_modifier_down(ctx: &Context, key_modifier: KeyModifier) -> bool {
    input::is_key_modifier_down(ctx, key_modifier)
}

pub fn is_key_pressed(ctx: &Context, key: Key) -> bool {
    input::is_key_pressed(ctx, key)
}

pub fn is_no_key_modifiers(ctx: &Context) -> bool {
    input::is_key_modifier_up(ctx, KeyModifier::Shift)
        && input::is_key_modifier_up(ctx, KeyModifier::Alt)
        && input::is_key_modifier_up(ctx, KeyModifier::Ctrl)
}

pub fn get_direction_keys_down(ctx: &Context) -> Option<Direction> {
    let key_down = |np: Key, n: Key| -> bool {
        input::is_key_down(ctx, np)
            || (input::is_key_down(ctx, n) && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    };
    if key_down(Key::NumPad5, Key::Num5) {
        return Some(Direction::Here);
    }
    if key_down(Key::NumPad7, Key::Num7) {
        return Some(Direction::NorthWest);
    }
    if key_down(Key::NumPad9, Key::Num9) {
        return Some(Direction::NorthEast);
    }
    if key_down(Key::NumPad3, Key::Num3) {
        return Some(Direction::SouthEast);
    }
    if key_down(Key::NumPad1, Key::Num1) {
        return Some(Direction::SouthWest);
    }
    let key_down = |k1: Key, k2: Key, n: Key| -> bool {
        input::is_key_down(ctx, k1)
            || input::is_key_down(ctx, k2)
            || (input::is_key_down(ctx, n) && input::is_key_modifier_up(ctx, KeyModifier::Shift))
    };
    let (mut moving_x, mut moving_y) = (0i8, 0i8);
    if key_down(Key::Up, Key::NumPad8, Key::Num8) {
        moving_y -= 1;
    }
    if key_down(Key::Down, Key::NumPad2, Key::Num2) {
        moving_y += 1;
    }
    if key_down(Key::Left, Key::NumPad4, Key::Num4) {
        moving_x -= 1;
    }
    if key_down(Key::Right, Key::NumPad6, Key::Num6) {
        moving_x += 1;
    }
    match (moving_x, moving_y) {
        (-1, -1) => Some(Direction::NorthWest),
        (-1, 1) => Some(Direction::SouthWest),
        (1, -1) => Some(Direction::NorthEast),
        (1, 1) => Some(Direction::SouthEast),
        (1, 0) => Some(Direction::East),
        (-1, 0) => Some(Direction::West),
        (0, -1) => Some(Direction::North),
        (0, 1) => Some(Direction::South),
        _ => None,
    }
}
