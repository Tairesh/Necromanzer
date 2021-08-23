#![windows_subsystem = "windows"]

mod colors;
mod settings;

extern crate serde;
extern crate tetra;

use colors::Colors;
use settings::Settings;
use tetra::graphics::{self, ImageData};
use tetra::input::Key;
use tetra::window::{set_icon, set_title};
use tetra::{time, Context, ContextBuilder, Event, State};

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

struct GameState {
    default_title: String,
    settings: Settings,
}

impl GameState {
    pub fn new(settings: Settings) -> GameState {
        GameState {
            default_title: format!("{} {}", TITLE, *VERSION),
            settings,
        }
    }
}

impl Drop for GameState {
    fn drop(&mut self) {
        self.settings.save();
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.settings.show_fps {
            let title = format!(
                "{} ({} FPS)",
                self.default_title,
                time::get_fps(ctx).round()
            );
            set_title(ctx, title);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::LAVENDER);
        Ok(())
    }
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                self.settings.show_fps = !self.settings.show_fps;
                if !self.settings.show_fps {
                    set_title(ctx, &self.default_title);
                }
            }
            Event::KeyPressed { key } => println!("{}", serde_json::to_string(&key).unwrap()),
            _ => {}
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    let settings = Settings::load().unwrap();
    let mut ctx = ContextBuilder::new(
        format!("{} {}", TITLE, *VERSION),
        settings.width as i32,
        settings.height as i32,
    );
    ctx.quit_on_escape(true).show_mouse(true).vsync(true);
    let mut ctx = if settings.fullscreen && settings.borderless {
        ctx.resizable(true).maximized(true).borderless(true)
    } else if settings.fullscreen && !settings.borderless {
        ctx.fullscreen(true)
    } else {
        ctx.resizable(true)
    }
    .build()?;
    let mut icon = ImageData::from_file("res/img/zombie.png")?;
    set_icon(&mut ctx, &mut icon)?;
    ctx.run(|_| Ok(GameState::new(settings)))
}
