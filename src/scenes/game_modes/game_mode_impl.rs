use map::tile::Tile;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::Context;

pub trait GameModeImpl {
    fn draw_cursors(&self) -> bool;
    fn draw_cursor_here(&self, tile: &Tile) -> bool;
    fn update(&self, game: &mut Game, ctx: &mut Context) -> SomeTransitions;
}
