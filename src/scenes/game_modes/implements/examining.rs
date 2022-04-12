use super::super::GameMode;
use input;
use scenes::implements::Game;
use scenes::transition::SomeTransitions;
use tetra::input::Key;
use tetra::Context;

pub fn update(game: &mut Game, ctx: &mut Context) -> SomeTransitions {
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
            println!("{}", this_is);
        }
    } else if game.selected.is_some() {
        game.mode = GameMode::Default;
        game.selected = None;
    }
    None
}
