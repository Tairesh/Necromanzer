use map::tile::Tile;

#[derive(Debug)]
pub enum GameMode {
    Default,
    Examining,
    Wielding,
    Dropping,
    Digging,
}

impl GameMode {
    pub fn draw_cursors(&self) -> bool {
        match self {
            GameMode::Default => false,
            GameMode::Examining | GameMode::Wielding | GameMode::Dropping | GameMode::Digging => {
                true
            }
        }
    }

    pub fn cursor_here(&self, tile: &Tile) -> bool {
        match self {
            GameMode::Wielding => !tile.items.is_empty(),
            GameMode::Dropping => tile.terrain.is_walkable(),
            GameMode::Digging => tile.terrain.is_diggable(),
            GameMode::Examining | GameMode::Default => false,
        }
    }
}
