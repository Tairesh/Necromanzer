use fps::FpsCounter;

use scene_manager::{CallResult, SceneManager};
use sdl2::event::{Event, WindowEvent};
use sdl2::image::LoadSurface;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::{FullscreenType, WindowContext, WindowPos};
use settings::Settings;
use sprite::{Sprite, SpriteT, SpritesManager};
use std::error::Error;

const FPS_LOCK: u32 = 60;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum WindowMode {
    Fullscreen,
    Borderless,
    Window,
}

pub struct EngineContext {
    default_title: String,
    default_size: (u32, u32),
    pub canvas: WindowCanvas,
    pub sprite_manager: SpritesManager,
    pub fps_counter: FpsCounter,
    pub window_mode: WindowMode,
}

impl EngineContext {
    pub fn new(
        canvas: WindowCanvas,
        default_size: (u32, u32),
        texture_creator: TextureCreator<WindowContext>,
        fps_counter: FpsCounter,
        window_mode: WindowMode,
    ) -> EngineContext {
        EngineContext {
            default_title: canvas.window().title().to_string(),
            default_size,
            canvas,
            sprite_manager: SpritesManager::new(
                sdl2::ttf::init().expect("Can't init sdl2::ttf!"),
                texture_creator,
            ),
            fps_counter,
            window_mode,
        }
    }

    pub fn change_window_mode(&mut self, new_mode: WindowMode) -> bool {
        if self.window_mode != new_mode {
            let window = self.canvas.window_mut();
            match new_mode {
                WindowMode::Fullscreen => {
                    window.set_fullscreen(FullscreenType::Desktop).ok();
                }
                WindowMode::Borderless => {
                    if window.fullscreen_state() == FullscreenType::Desktop {
                        window.set_fullscreen(FullscreenType::Off).ok();
                    }
                    window.set_bordered(false);
                    window.maximize();
                }
                WindowMode::Window => {
                    if window.fullscreen_state() == FullscreenType::Desktop {
                        window.set_fullscreen(FullscreenType::Off).ok();
                    }
                    window.set_bordered(true);
                    window
                        .set_size(self.default_size.0, self.default_size.1)
                        .ok();
                    window.set_position(WindowPos::Centered, WindowPos::Centered);
                }
            }
            self.window_mode = new_mode;
            true
        } else {
            false
        }
    }

    pub fn set_show_fps(&mut self, value: bool) {
        self.fps_counter.show_fps = value;
        if !value {
            self.canvas
                .window_mut()
                .set_title(self.default_title.as_str())
                .ok();
        }
    }

    pub fn draw_sprites(&mut self, sprites: &[Sprite]) -> Result<(), String> {
        for sprite in sprites.iter() {
            let texture = self.sprite_manager.render_sprite(sprite);
            let position: (i32, i32) = sprite.position();
            let size = texture.query();
            self.canvas.copy(
                texture,
                None,
                Some(Rect::new(position.0, position.1, size.width, size.height)),
            )?;
        }
        Ok(())
    }
}

pub struct Engine<'a> {
    title: &'a str,
    pub settings: Settings,
    pub sdl: sdl2::Sdl,
    pub scene_manager: SceneManager,
    pub context: Option<EngineContext>,
}

impl<'a> Engine<'a> {
    pub fn new(title: &'a str) -> Result<Engine<'a>, String> {
        Ok(Engine {
            title,
            settings: Settings::load()?,
            sdl: sdl2::init()?,
            scene_manager: SceneManager::new(),
            context: None,
        })
    }

    fn create_window(&mut self) -> Result<WindowCanvas, Box<dyn Error>> {
        let video = self.sdl.video()?;

        let mut window_builder =
            video.window(self.title, self.settings.width, self.settings.height);
        window_builder.hidden().resizable().allow_highdpi();
        let mut window = if self.settings.fullscreen {
            if self.settings.borderless {
                window_builder.borderless().maximized()
            } else {
                window_builder.fullscreen_desktop()
            }
        } else {
            window_builder.position_centered()
        }
        .build()?;

        let icon: Surface =
            LoadSurface::from_file("res/img/zombie.png").expect("Can't load resources!");
        window.set_icon(icon);
        window.set_minimum_size(800, 600).ok();
        window.set_maximum_size(1920, 1280).ok();
        window.show();
        Ok(window.into_canvas().accelerated().build()?)
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let canvas = self.create_window().unwrap();
        let texture_creator = canvas.texture_creator();
        let window_mode = match (self.settings.fullscreen, self.settings.borderless) {
            (true, true) => WindowMode::Borderless,
            (true, false) => WindowMode::Fullscreen,
            _ => WindowMode::Window,
        };
        self.context = Some(EngineContext::new(
            canvas,
            (self.settings.width, self.settings.height),
            texture_creator,
            FpsCounter::new(FPS_LOCK, self.sdl.timer()?),
            window_mode,
        ));
        self.scene_manager.on_open(self.context.as_mut().unwrap());
        let mut event_pump = self.sdl.event_pump()?;
        self.context
            .as_mut()
            .unwrap()
            .set_show_fps(self.settings.show_fps);
        'running: loop {
            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::Window {
                        win_event: WindowEvent::Resized { .. },
                        ..
                    } => {
                        let (w, h) = self.context.as_ref().unwrap().canvas.window().size();
                        self.settings.width = w;
                        self.settings.height = h;
                        self.scene_manager.on_resize(self.context.as_mut().unwrap());
                        // println!("Window resized to {}x{}", w, h);
                    }
                    _ => match self
                        .scene_manager
                        .call(self.context.as_mut().unwrap(), &event)
                    {
                        CallResult::ChangeScene(new_scene) => {
                            self.scene_manager
                                .change_scene(self.context.as_mut().unwrap(), new_scene.as_str());
                        }
                        CallResult::ChangeWindowMode(new_mode) => {
                            if self.context.as_mut().unwrap().change_window_mode(new_mode) {
                                self.scene_manager.on_resize(self.context.as_mut().unwrap());
                                match new_mode {
                                    WindowMode::Fullscreen => {
                                        self.settings.borderless = false;
                                        self.settings.fullscreen = true;
                                    }
                                    WindowMode::Borderless => {
                                        self.settings.borderless = true;
                                        self.settings.fullscreen = true;
                                    }
                                    WindowMode::Window => {
                                        let (w, h) =
                                            self.context.as_ref().unwrap().canvas.window().size();
                                        self.settings.borderless = false;
                                        self.settings.fullscreen = false;
                                        self.settings.width = w;
                                        self.settings.height = h;
                                    }
                                }
                            }
                        }
                        CallResult::SystemExit => break 'running,
                        CallResult::DoNothing => {}
                    },
                }
            }

            let (need_render, elapsed_time, show_fps) =
                self.context.as_mut().unwrap().fps_counter.tick();

            if need_render {
                if let Some(fps) = show_fps {
                    let title = format!("{} ({} FPS)", self.title, fps);
                    self.context
                        .as_mut()
                        .unwrap()
                        .canvas
                        .window_mut()
                        .set_title(title.as_str())
                        .ok();
                }
                // Process next frame and exit if `Ok(false)` is returned
                if !self.on_update(elapsed_time)? {
                    break 'running;
                }
            }
        }
        self.settings.save();
        Ok(())
    }

    fn on_update(&mut self, elapsed_time: f64) -> Result<bool, Box<dyn Error>> {
        let context = self.context.as_mut().unwrap();
        self.scene_manager.on_update(context, elapsed_time);
        context.canvas.present();
        Ok(true)
    }
}
