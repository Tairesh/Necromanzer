#![windows_subsystem = "windows"]
mod colors;
mod engine;
mod fps;
mod scene_manager;
mod scenes;
mod settings;
mod sprite;

extern crate sdl2;

use engine::Engine;

const TITLE: &str = "Necromanzer";
const CARGO_VERSION: &str = env!("CARGO_PKG_VERSION");
const BUILD_VERSION: &str = "dirty";
lazy_static::lazy_static! {
    static ref VERSION: String = if BUILD_VERSION == "stable" {
        format!("v{}", CARGO_VERSION)
    } else {
        format!("v{}-{}", CARGO_VERSION, BUILD_VERSION)
    };
}

pub fn main() {
    let title = format!("{} {}", TITLE, *VERSION);
    let mut engine = Engine::new(title.as_str()).unwrap();
    engine.start().ok();
}
