use super::GameMode;
use enum_dispatch::enum_dispatch;
use game::World;
use geometry::point::Point;
use scenes::game_modes::implements::*;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::graphics::Color;
use tetra::Context;

#[enum_dispatch]
pub trait GameModeImpl {
    fn cursors(&self, _world: &World) -> Vec<(Point, Color)> {
        vec![]
    }
    fn can_push(&self, _world: &World) -> Result<(), String> {
        Ok(())
    }
    fn update(&mut self, ctx: &mut Context, game: &mut Game) -> SomeTransitions;
    fn draw(&mut self, _ctx: &mut Context, _game: &mut Game) {}
}
