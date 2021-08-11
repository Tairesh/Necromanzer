use fps::FpsCounter;

use scene::{CallResult, MainMenu, Scene, SceneT};
use sdl2::event::{Event, WindowEvent};
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use settings::Settings;
use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use textures::TextureManager;

const FPS_LOCK: u32 = 60;

pub struct Engine<'a> {
    title: &'a str,
    pub settings: Settings,
    pub ctx: sdl2::Sdl,
    pub canvas: Option<WindowCanvas>,
    pub scene: Option<Scene>,
    pub texture_manager: Option<TextureManager>,
}

impl<'a> Engine<'a> {
    pub fn new(title: &'a str) -> Result<Engine<'a>, String> {
        Ok(Engine {
            title,
            settings: Settings::load()?,
            ctx: sdl2::init()?,
            canvas: None,
            scene: None,
            texture_manager: None,
        })
    }

    fn create_window(&mut self) -> Result<WindowCanvas, Box<dyn Error>> {
        let video = self.ctx.video()?;
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
        window.set_minimum_size(640, 480).ok();
        window.set_maximum_size(1920, 1280).ok();
        window.show();
        Ok(window.into_canvas().accelerated().build()?)
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.canvas = Some(self.create_window().unwrap());
        self.scene = Some(MainMenu {}.into());
        self.texture_manager = Some(TextureManager::new(
            sdl2::ttf::init()?,
            self.canvas.as_ref().unwrap().texture_creator(),
        ));
        let mut event_pump = self.ctx.event_pump()?;
        let mut fps_counter = FpsCounter::new(self.ctx.timer()?);
        let ns_per_frame: Duration = Duration::new(0, 1_000_000_000u32 / FPS_LOCK);
        'running: loop {
            let start = Instant::now();
            let elapsed_time = fps_counter.update(self.settings.show_fps);
            if self.settings.show_fps && fps_counter.time_acc() >= 1.0 {
                let fps = fps_counter.fps();
                let title = format!("{} ({} FPS)", self.title, fps.round() as u32);
                // This fails silently on error
                self.canvas
                    .as_mut()
                    .unwrap()
                    .window_mut()
                    .set_title(title.as_str())
                    .ok();
                fps_counter.reset_average();
            }

            // Process next frame and exit if `Ok(false)` is returned
            if !self.on_update(elapsed_time)? {
                break 'running;
            }

            // Handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    Event::Window {
                        win_event: WindowEvent::Resized { .. },
                        ..
                    } => {
                        let (w, h) = self.canvas.as_ref().unwrap().window().size();
                        self.settings.width = w;
                        self.settings.height = h;
                        // println!("Window resized to {}x{}", w, h);
                    }
                    _ => match self.scene.as_mut().unwrap().call(&event) {
                        CallResult::ChangeScene(new_scene) => {
                            self.scene = Some(new_scene);
                        }
                        CallResult::DoNothing => {}
                    },
                }
            }

            // Framerate cap
            let next_render_step = start + ns_per_frame;
            let now = Instant::now();
            if next_render_step >= now {
                sleep(next_render_step - now);
            }
        }
        self.settings.save();
        Ok(())
    }

    fn on_update(&mut self, elapsed_time: f64) -> Result<bool, Box<dyn Error>> {
        let scene = self.scene.as_mut().unwrap();
        let canvas = self.canvas.as_mut().unwrap();
        let textures = self.texture_manager.as_mut().unwrap();
        scene.update(canvas, textures, elapsed_time);
        canvas.present();
        Ok(true)
    }
}
