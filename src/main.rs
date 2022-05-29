#![windows_subsystem = "windows"]

extern crate arrayvec;
extern crate core;
extern crate enum_dispatch;
extern crate enum_iterator;
extern crate once_cell;
extern crate phf;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate serde_with;
extern crate tetra;
extern crate time;

mod app;
mod assets;
mod colors;
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
