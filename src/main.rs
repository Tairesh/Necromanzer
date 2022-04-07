#![windows_subsystem = "windows"]

extern crate serde_json;
extern crate tetra;

mod app;
mod settings;
mod window;

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
