use assets::Assets;
use settings::game::GameSettings;
use std::rc::Rc;
use tetra::input::Key;
use tetra::{Context, Event, State, TetraError};

pub struct App {
    #[allow(dead_code)]
    assets: Rc<Assets>,
    settings: GameSettings,
}

impl App {
    pub fn new(ctx: &mut Context, settings: GameSettings) -> tetra::Result<Self> {
        let mut app = Self {
            settings,
            assets: Rc::new(Assets::load(ctx)?),
        };
        app.on_open(ctx);
        Ok(app)
    }

    fn on_open(&mut self, ctx: &mut Context) {
        // TODO current_scene.op_open()
        self.on_resize(ctx);
    }

    fn on_resize(&mut self, _ctx: &mut Context) {
        // TODO: repositionate sprites
    }
}

impl State for App {
    fn draw(&mut self, _ctx: &mut Context) -> Result<(), TetraError> {
        if self.settings.show_fps {
            // TODO: draw fps sprite
        }
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        match event {
            Event::KeyPressed { key: Key::F2 } => {
                self.settings.show_fps = !self.settings.show_fps;
            }
            Event::Resized { width, height } => {
                self.settings.window_settings.width = width;
                self.settings.window_settings.height = height;
                self.on_resize(ctx);
            }
            _ => {}
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.settings.save();
        // TODO: save savefile probably
    }
}
