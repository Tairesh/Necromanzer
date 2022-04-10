#![windows_subsystem = "windows"]

extern crate core;
extern crate num_enum;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate tetra;
extern crate time;
extern crate variant_count;

mod actions;
mod app;
mod assets;
mod avatar;
mod colors;
mod enums;
mod geometry;
mod human;
mod input;
mod map;
mod savefile;
mod scenes;
mod settings;
mod sprites;
mod window;
mod world;

const NAME: &str = "NecromanZer";
const VERSION: &str = concat!(
    "v",
    env!("CARGO_PKG_VERSION"),
    env!("NECROMANZER_VERSION_POSTFIX")
);

fn main() -> tetra::Result {
    let settings = settings::game::GameSettings::load()?;
    window::create_context(format!("{} {}", NAME, VERSION), &settings.window_settings)?
        .run(|ctx| app::App::new(ctx, settings))
}
