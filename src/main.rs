#![windows_subsystem = "windows"]
mod colors;
// mod engine;
// mod fps;
// mod scene_manager;
// mod scenes;
mod settings;
// mod sprite;
//
extern crate serde;
extern crate tetra;

//
// use engine::Engine;
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

// pub fn main() {
//     let title = format!("{} {}", TITLE, *VERSION);
//     let mut engine = Engine::new(title.as_str()).unwrap();
//     engine.start().ok();
// }

struct GameState {
    default_title: String,
    settings: Settings,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            default_title: format!("{} {}", TITLE, *VERSION),
            settings: Settings::load().unwrap(),
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
            set_title(
                ctx,
                format!(
                    "{} {} ({} FPS)",
                    TITLE,
                    *VERSION,
                    time::get_fps(ctx).round()
                ),
            )
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, colors::rgb(colors::LAVENDER));
        Ok(())
    }
    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                self.settings.show_fps = !self.settings.show_fps;
                if !self.settings.show_fps {
                    set_title(ctx, self.default_title.as_str());
                }
            }
            Event::KeyPressed { key } => println!("{}", serde_json::to_string(&key).unwrap()),
            _ => {}
        }
        Ok(())
    }
}

fn main() -> tetra::Result {
    let mut ctx = ContextBuilder::new(format!("{} {}", TITLE, *VERSION), 1024, 768)
        .quit_on_escape(true)
        .show_mouse(true)
        .build()?;
    let mut icon = ImageData::from_file("res/img/zombie.png")?;
    set_icon(&mut ctx, &mut icon)?;
    ctx.run(|_| Ok(GameState::new()))
}
