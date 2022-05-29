#![windows_subsystem = "windows"]

mod app;
mod assets;
mod colors;
mod fov;
mod game;
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
