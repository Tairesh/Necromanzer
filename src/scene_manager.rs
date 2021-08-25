use settings::{Settings, WindowMode};
use sprites::sprite::Sprite;
use tetra::input::Key;
use tetra::{time, window, Event, TetraError};
use tetra::{Context, State};
use {TITLE, VERSION};

pub trait Scene {
    fn on_button_click(&mut self, _ctx: &mut Context, _btn_id: &str) -> Option<Transition> {
        None
    }
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        let mut btn_clicked = None;
        for sprite in self.sprites().iter_mut() {
            if let Some(btn_id) = sprite.update(ctx) {
                btn_clicked = Some(btn_id);
            }
        }
        if let Some(btn_id) = btn_clicked {
            if let Some(t) = self.on_button_click(ctx, btn_id.as_str()) {
                return Ok(t);
            }
        }
        Ok(Transition::None)
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.sprites().iter().any(|s| s.dirty()) {
            self.clear(ctx)?;
        }
        Ok(())
    }
    fn clear(&mut self, ctx: &mut Context) -> tetra::Result {
        for sprite in self.sprites().iter_mut() {
            sprite.draw(ctx);
        }
        Ok(())
    }
    fn on_resize(&mut self, ctx: &mut Context) -> tetra::Result {
        for sprite in self.sprites().iter_mut() {
            sprite.calc_position(ctx);
        }
        self.clear(ctx)
    }
    fn sprites(&mut self) -> &mut Vec<Box<dyn Sprite>>;
    fn on_open(&mut self, ctx: &mut Context) -> tetra::Result {
        self.on_resize(ctx)
    }
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
                    self.scenes.last_mut().unwrap().on_open(ctx)?;
                }
                Transition::Pop => {
                    self.scenes.pop();
                    if let Some(new_scene) = self.scenes.last_mut() {
                        new_scene.on_open(ctx)?;
                    }
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
                if let Some(scene) = self.scenes.last_mut() {
                    scene.on_resize(ctx)?;
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
