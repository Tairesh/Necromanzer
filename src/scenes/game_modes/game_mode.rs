use enum_dispatch::enum_dispatch;
use scenes::game_modes::implements::*;

#[enum_dispatch(GameModeImpl)]
pub enum GameMode {
    Walking,
    Examining,
    Wielding,
    Dropping,
    Digging,
    Observing,
    Reading,
    Animate,
}
