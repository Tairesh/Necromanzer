use enum_dispatch::enum_dispatch;
use scenes::game_modes::implements::digging::Digging;
use scenes::game_modes::implements::dropping::Dropping;
use scenes::game_modes::implements::examining::Examining;
use scenes::game_modes::implements::observing::Observing;
use scenes::game_modes::implements::walking::Walking;
use scenes::game_modes::implements::wielding::Wielding;

#[enum_dispatch(GameModeImpl)]
#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Walking,
    Examining,
    Wielding,
    Dropping,
    Digging,
    Observing,
}
