#![windows_subsystem = "windows"]
mod engine;
mod fps;
mod scene;
mod settings;
mod textures;

extern crate sdl2;

use engine::Engine;
use std::fs;

const CARGO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const TITLE: &'static str = "Necromanzer";
lazy_static::lazy_static! {
    static ref VERSION: String = format!(
        "v{}-{}",
        CARGO_VERSION,
        fs::read_to_string("version.txt").unwrap_or("dirty".to_string())
    );
}

pub fn main() {
    let title = format!("{} {}", TITLE, *VERSION);
    let mut engine = Engine::new(title.as_str()).unwrap();
    engine.start().ok();
}
