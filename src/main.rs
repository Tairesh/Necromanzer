#![windows_subsystem = "windows"]

extern crate arrayvec;
extern crate core;
extern crate enum_dispatch;
extern crate num_enum;
extern crate once_cell;
extern crate phf;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate tetra;
extern crate time;
extern crate variant_count;

mod app;
mod assets;
mod colors;
mod enums_iter;
mod fov;
mod game;
mod geometry;
mod input;
mod savefile;
mod scenes;
mod settings;
mod ui;
mod window;

const NAME: &str = "NecromanZer";
const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    env!("NECROMANZER_VERSION_POSTFIX")
);

fn main() -> tetra::Result {
    window::create_context(format!("{} {}", NAME, VERSION))?.run(app::App::new)
}
