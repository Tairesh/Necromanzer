use settings::{Settings, WindowMode};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::Key;
use tetra::window::WindowPos;
use tetra::{time, window, Event, TetraError};
use tetra::{Context, State};
use {TITLE, VERSION};

pub(crate) fn update_sprites<T: Scene>(scene: &mut T, ctx: &mut Context) -> Option<Transition> {
    if let Some(sprites) = scene.sprites() {
        let mut btn_clicked = None;
        for sprite in sprites.iter_mut() {
            if let Some(btn_id) = sprite.update(ctx) {
                btn_clicked = Some(btn_id);
            }
        }
        if let Some(btn_id) = btn_clicked {
            let t = scene.on_button_click(ctx, btn_id.as_str());
            if t.is_some() {
                return t;
            }
        }
    }
    None
}

pub trait Scene {
    fn on_button_click(&mut self, _ctx: &mut Context, _btn_id: &str) -> Option<Transition> {
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(sprites) = self.sprites() {
            if sprites.iter().any(|s| s.dirty()) {
                self.redraw_sprites(ctx)?;
            }
        }
        Ok(())
    }
    fn redraw_sprites(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(sprites) = self.sprites() {
            for sprite in sprites.iter_mut() {
                if sprite.visible() {
                    sprite.draw(ctx);
                }
            }
        }
        Ok(())
    }
    fn on_resize(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(sprites) = self.sprites() {
            let window_size = window::get_size(ctx);
            for sprite in sprites.iter_mut() {
                let size = sprite.calc_size(ctx);
                let rect = sprite.calc_rect(size, window_size);
                sprite.set_rect(rect);
            }
            self.redraw_sprites(ctx)
        } else {
            Ok(())
        }
    }
    fn sprites(&mut self) -> Option<&mut Vec<Box<dyn Sprite>>> {
        None
    }
    fn on_open(&mut self, ctx: &mut Context) -> tetra::Result {
        self.on_resize(ctx)
    }
}

pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>), // pop and push
    ChangeWindowMode(WindowMode),
    Quit,
}

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    settings: Rc<RefCell<Settings>>,
    default_title: String,
}

impl SceneManager {
    pub fn new(initial_scene: Box<dyn Scene>, settings: Rc<RefCell<Settings>>) -> SceneManager {
        SceneManager {
            scenes: vec![initial_scene],
            settings,
            default_title: format!("{} {}", TITLE, *VERSION),
        }
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.settings.borrow().show_fps {
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
                Transition::Replace(s) => {
                    self.scenes.pop();
                    if let Some(new_scene) = self.scenes.last_mut() {
                        new_scene.on_open(ctx)?;
                    }
                    self.scenes.push(s);
                    self.scenes.last_mut().unwrap().on_open(ctx)?;
                }
                Transition::Quit => window::quit(ctx),
                Transition::ChangeWindowMode(wm) => {
                    let mut settings = self.settings.borrow_mut();
                    if wm != settings.window_mode() {
                        match wm {
                            WindowMode::Fullscreen => {
                                settings.fullscreen = true;
                                settings.borderless = false;
                                window::set_fullscreen(ctx, true)?;
                            }
                            WindowMode::Borderless => {
                                settings.fullscreen = true;
                                settings.borderless = true;
                                if window::is_fullscreen(ctx) {
                                    window::set_fullscreen(ctx, false)?;
                                }
                                window::set_bordered(ctx, false);
                                window::maximize(ctx);
                            }
                            WindowMode::Window => {
                                settings.fullscreen = false;
                                settings.borderless = false;
                                if window::is_fullscreen(ctx) {
                                    window::set_fullscreen(ctx, false)?;
                                }
                                window::set_bordered(ctx, true);
                                window::set_size(
                                    ctx,
                                    settings.width as i32,
                                    settings.height as i32,
                                )?;
                                window::set_position(ctx, WindowPos::Centered, WindowPos::Centered);
                            }
                        }
                    }
                }
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
        let mut settings = self.settings.borrow_mut();
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                settings.show_fps = !settings.show_fps;
                if !settings.show_fps {
                    window::set_title(ctx, &self.default_title);
                }
            }
            Event::FocusGained | Event::Restored => {
                if let Some(scene) = self.scenes.last_mut() {
                    scene.redraw_sprites(ctx)?;
                }
            }
            Event::Resized { width, height } => {
                if !settings.fullscreen {
                    settings.width = width as u32;
                    settings.height = height as u32;
                    settings.validate();
                    if settings.width as i32 != width || settings.height as i32 != height {
                        window::set_size(ctx, settings.width as i32, settings.height as i32).ok();
                        window::set_position(ctx, WindowPos::Centered, WindowPos::Centered);
                    }
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
        self.settings.borrow_mut().save();
    }
}
