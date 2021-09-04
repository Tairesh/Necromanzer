#![windows_subsystem = "windows"]

mod action;
mod assets;
mod avatar;
mod colors;
mod direction;
mod geometry;
mod human;
mod input;
mod map;
mod savefile;
mod scenes;
mod settings;
mod sprites;
mod world;

extern crate chrono;
extern crate enum_dispatch;
extern crate rand;
extern crate serde;
extern crate tetra;

use assets::Assets;
use scenes::main_menu::MainMenu;
use scenes::manager::{Scene, SceneManager};
use settings::Settings;
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
    let settings = Rc::new(RefCell::new(Settings::load()?));
    let mut ctx = ContextBuilder::new(
        format!("{} {}", TITLE, *VERSION),
        settings.borrow().width as i32,
        settings.borrow().height as i32,
    );
    ctx.show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    if settings.borrow().fullscreen {
        ctx.fullscreen(true);
    }
    let mut ctx = ctx.build()?;
    let mut icon = ImageData::from_file_data(include_bytes!("../res/img/zombie.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1024, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;

    ctx.run(|ctx| {
        let mut scene = MainMenu::new(Rc::new(RefCell::new(Assets::new(ctx)?)), settings.clone());
        scene.on_open(ctx);
        Ok(SceneManager::new(Box::new(scene), settings.clone()))
    })
}
