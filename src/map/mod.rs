pub mod chunk;
pub mod item;
pub mod pos;
mod terrains;
pub mod tile;

#[derive(Debug)]
pub enum Passage {
    Passable(f32), // ticks to pass (for 2-legged human)
    Unpassable,
}
