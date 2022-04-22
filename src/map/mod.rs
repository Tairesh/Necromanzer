pub mod chunk;
pub mod item;
pub mod pos;
pub mod terrains;
pub mod tile;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Passage {
    Passable(f32), // ticks to pass (for 2-legged human)
    Unpassable,
}
