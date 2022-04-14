use super::GameMode;
use enum_dispatch::enum_dispatch;
use geometry::Vec2;
use scenes::game_modes::implements::digging::Digging;
use scenes::game_modes::implements::dropping::Dropping;
use scenes::game_modes::implements::examining::Examining;
use scenes::game_modes::implements::observing::Observing;
use scenes::game_modes::implements::walking::Walking;
use scenes::game_modes::implements::wielding::Wielding;
use scenes::game_modes::SomeResults;
use scenes::implements::Game;
use tetra::graphics::Color;
use tetra::Context;

#[enum_dispatch]
pub trait GameModeImpl {
    fn cursors(&self, _game: &Game) -> Vec<(Vec2, Color)> {
        vec![]
    }
    fn can_push(&self, _game: &Game) -> Result<(), String> {
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context) -> SomeResults;
}
