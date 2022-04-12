use super::GameModeImpl;
use map::tile::Tile;
use scenes::game_modes::implements;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::Context;

#[derive(Debug, Copy, Clone)]
pub enum GameMode {
    Default,
    Examining,
    Wielding,
    Dropping,
    Digging,
}

impl GameMode {
    pub fn draw_cursors(&self) -> bool {
        self.to_struct().draw_cursors()
    }

    pub fn draw_cursor_here(&self, tile: &Tile) -> bool {
        self.to_struct().draw_cursor_here(tile)
    }

    pub fn update(&self, game: &mut Game, ctx: &mut Context) -> SomeTransitions {
        self.to_struct().update(game, ctx)
    }

    fn to_struct(self) -> Box<dyn GameModeImpl> {
        match self {
            GameMode::Default => Box::new(implements::default::Default {}),
            GameMode::Examining => Box::new(implements::examining::Examining {}),
            GameMode::Wielding => Box::new(implements::wielding::Wielding {}),
            GameMode::Dropping => Box::new(implements::dropping::Dropping {}),
            GameMode::Digging => Box::new(implements::digging::Digging {}),
        }
    }
}
