use settings::{Settings, WindowMode};
use tetra::input::Key;
use tetra::{time, window, Event, TetraError};
use tetra::{Context, State};
use {TITLE, VERSION};

pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result;
}

#[allow(dead_code)]
pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    ChangeWindowMode(WindowMode),
    Quit,
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    settings: Settings,
    default_title: String,
}

impl SceneManager {
    pub fn new(initial_scene: Box<dyn Scene>, settings: Settings) -> SceneManager {
        SceneManager {
            scenes: vec![initial_scene],
            settings,
            default_title: format!("{} {}", TITLE, *VERSION),
        }
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.settings.show_fps {
            let title = format!(
                "{} ({} FPS)",
                self.default_title,
                time::get_fps(ctx).round()
            );
            window::set_title(ctx, title);
        }

        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(s) => {
                    self.scenes.push(s);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
                Transition::Quit => window::quit(ctx),
                Transition::ChangeWindowMode(_) => {}
            },
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => active_scene.draw(ctx)?,
            None => window::quit(ctx),
        }

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                self.settings.show_fps = !self.settings.show_fps;
                if !self.settings.show_fps {
                    window::set_title(ctx, &self.default_title);
                }
            }
            Event::Resized { width, height } => {
                self.settings.width = width as u32;
                self.settings.height = height as u32;
                self.settings.validate();
                if self.settings.width as i32 != width || self.settings.height as i32 != height {
                    window::set_size(ctx, self.settings.width as i32, self.settings.height as i32)
                        .ok();
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Drop for SceneManager {
    fn drop(&mut self) {
        self.settings.save();
    }
}
