use super::super::GameMode;
use colors::Colors;
use input;
use map::terrains::Terrain;
use map::tile::Tile;
use scenes::game_modes::GameModeImpl;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::input::Key;
use tetra::Context;

pub struct Examining {}

impl From<Examining> for GameMode {
    fn from(_: Examining) -> Self {
        GameMode::Examining
    }
}

impl GameModeImpl for Examining {
    fn draw_cursors(&self) -> bool {
        true
    }

    fn draw_cursor_here(&self, tile: &Tile) -> bool {
        matches!(tile.terrain, Terrain::Grave(..))
    }

    fn update(&self, game: &mut Game, ctx: &mut Context) -> SomeTransitions {
        if input::is_key_pressed(ctx, Key::Escape) {
            game.mode = GameMode::Default;
            game.selected = None;
        }
        if let Some(dir) = input::get_direction_keys_down(ctx) {
            if game.selected.is_none() {
                game.select(dir);
                let pos = game.world.player().pos + dir;
                let tile = game.world.load_tile(pos);
                let mut this_is = tile.terrain.this_is();
                if !tile.items.is_empty() {
                    let items: Vec<String> = tile
                        .items
                        .iter()
                        .map(|item| item.item_type.name())
                        .collect();
                    this_is += " Here you see: ";
                    this_is += items.join(", ").as_str();
                }
                game.world.log(this_is, Colors::WHITE_SMOKE);
            }
        } else if game.selected.is_some() {
            game.mode = GameMode::Default;
            game.selected = None;
        }
        None
    }
}
