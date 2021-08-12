#![windows_subsystem = "windows"]
mod engine;
mod fps;
mod scene;
mod settings;
mod textures;

extern crate sdl2;

use engine::Engine;

const VERSION: &str = "v0.1.0";
const TITLE: &str = "Necromanzer";

pub fn main() {
    let title = format!("{} {}", TITLE, VERSION);
    let mut engine = Engine::new(title.as_str()).unwrap();
    engine.start().ok();
}
