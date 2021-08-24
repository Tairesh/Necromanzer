#![windows_subsystem = "windows"]

mod assets;
mod colors;
mod scene_manager;
mod scenes;
mod settings;

extern crate serde;
extern crate tetra;

use assets::Assets;
use scene_manager::SceneManager;
use scenes::main_menu::MainMenu;
use settings::{Settings, WindowMode};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::ImageData;
use tetra::window;
use tetra::ContextBuilder;

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

fn main() -> tetra::Result {
    let settings = Settings::load().unwrap();
    let mut ctx = ContextBuilder::new(
        format!("{} {}", TITLE, *VERSION),
        settings.width as i32,
        settings.height as i32,
    );
    ctx.quit_on_escape(true).show_mouse(true).vsync(true);
    let mut ctx = match settings.window_mode() {
        WindowMode::Fullscreen => ctx.fullscreen(true),
        WindowMode::Borderless => ctx.resizable(true).maximized(true).borderless(true),
        WindowMode::Window => ctx.resizable(true),
    }
    .build()?;
    let mut icon = ImageData::from_file("res/img/zombie.png")?;
    window::set_icon(&mut ctx, &mut icon)?;

    ctx.run(|ctx| {
        let scene = MainMenu::new(Rc::new(RefCell::new(Assets::new(ctx)?)))?;
        Ok(SceneManager::new(Box::new(scene), settings))
    })
}
