use fps::FpsCounter;

use scene_manager::{CallResult, SceneManager};
use sdl2::event::{Event, WindowEvent};
use sdl2::image::LoadSurface;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use settings::Settings;
use sprite::{SceneSprites, Sprite, SpritesManager};
use std::error::Error;

const FPS_LOCK: u32 = 60;

pub struct EngineContext {
    default_title: String,
    pub canvas: WindowCanvas,
    pub sprite_manager: SpritesManager,
    pub fps_counter: FpsCounter,
}

impl EngineContext {
    pub fn new(
        canvas: WindowCanvas,
        texture_creator: TextureCreator<WindowContext>,
        fps_counter: FpsCounter,
    ) -> EngineContext {
        EngineContext {
            default_title: canvas.window().title().to_string(),
            canvas,
            sprite_manager: SpritesManager::new(
                sdl2::ttf::init().expect("Can't init sdl2::ttf!"),
                texture_creator,
            ),
            fps_counter,
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

    pub fn draw_sprites(&mut self, data: &SceneSprites) -> Result<(), String> {
        for sprite in data.sprites.iter() {
            let (texture, position) = match sprite {
                Sprite::Image(img) => (self.sprite_manager.load_image(img), img.position),
                Sprite::Label(label) => (self.sprite_manager.render_text(label), label.position),
                Sprite::Button(button) => {
                    (self.sprite_manager.render_button(button), button.position)
                }
            };
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
        let window_size = if self.settings.fullscreen {
            let mode = video.current_display_mode(0)?;
            (mode.w as u32, mode.h as u32)
        } else {
            (self.settings.width, self.settings.height)
        };

        let mut window_builder = video.window(self.title, window_size.0, window_size.1);
        window_builder.hidden().allow_highdpi();
        let mut window = if self.settings.fullscreen {
            if self.settings.borderless {
                window_builder.borderless()
            } else {
                window_builder.fullscreen_desktop()
            }
        } else {
            window_builder.position_centered().resizable()
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
        self.context = Some(EngineContext::new(
            canvas,
            texture_creator,
            FpsCounter::new(FPS_LOCK, self.sdl.timer()?),
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
