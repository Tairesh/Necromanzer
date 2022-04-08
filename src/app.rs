use assets::Assets;
use scenes::game_scene::GameScene;
use scenes::scene::Scene;
use scenes::transition::Transition;
use settings::game::GameSettings;
use tetra::input::Key;
use tetra::{window, Context, Event, State};

pub struct App {
    pub assets: Assets,
    pub settings: GameSettings,
    scenes: Vec<Box<dyn Scene>>,
}

impl App {
    pub fn new(ctx: &mut Context, settings: GameSettings) -> tetra::Result<Self> {
        let mut app = Self {
            settings,
            assets: Assets::load(ctx)?,
            scenes: vec![],
        };
        app.scenes.push(GameScene::MainMenu.to_impl(&app, ctx));
        app.on_open(ctx);
        Ok(app)
    }

    fn current_scene(&mut self) -> Option<&mut Box<dyn Scene>> {
        self.scenes.last_mut()
    }

    fn on_open(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.current_scene() {
            scene.on_open(ctx);
        }
        self.on_resize(ctx);
    }

    fn on_resize(&mut self, ctx: &mut Context) {
        if let Some(scene) = self.current_scene() {
            let window_size = window::get_size(ctx);
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    sprite.borrow_mut().positionate(ctx, window_size);
                }
            }
            scene.on_resize(ctx);
        }
    }

    fn pop_scene(&mut self, ctx: &mut Context) {
        self.scenes.pop();
        self.on_open(ctx);
    }

    fn replace_scene(&mut self, ctx: &mut Context, scene: GameScene) {
        self.scenes.pop();
        self.push_scene(ctx, scene);
    }

    fn push_scene(&mut self, ctx: &mut Context, scene: GameScene) {
        self.scenes.push(scene.to_impl(self, ctx));
        self.on_open(ctx);
    }

    fn transit(&mut self, ctx: &mut Context, transition: Transition) {
        match transition {
            Transition::Push(s) => self.push_scene(ctx, s),
            Transition::Pop => self.pop_scene(ctx),
            Transition::Replace(s) => self.replace_scene(ctx, s),
            Transition::CustomEvent(event) => {
                if let Some(scene) = self.current_scene() {
                    for t in scene.custom_event(ctx, event.as_str()) {
                        self.transit(ctx, t);
                    }
                }
            }
            Transition::Quit => window::quit(ctx),
        }
    }
}

impl State for App {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(scene) = self.current_scene() {
            let mut button_clicked = None;
            let focused = scene
                .sprites()
                .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
                .unwrap_or(false);
            if let Some(sprites) = scene.sprites() {
                let mut blocked = Vec::with_capacity(sprites.len());
                for sprite in sprites.iter().rev() {
                    let mut sprite = sprite.borrow_mut();
                    if let Some(transition) = sprite.update(ctx, focused, &blocked) {
                        button_clicked = Some(transition);
                    }
                    if sprite.visible() && sprite.block_mouse() {
                        blocked.push(sprite.rect());
                    }
                }
            }
            if let Some(t) = button_clicked {
                self.transit(ctx, t);
            } else {
                for t in scene.update(ctx, focused) {
                    self.transit(ctx, t);
                }
            }
        } else {
            self.transit(ctx, Transition::Quit);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(scene) = self.current_scene() {
            scene.before_draw(ctx);
            if let Some(sprites) = scene.sprites() {
                for sprite in sprites.iter() {
                    let mut sprite = sprite.borrow_mut();
                    if sprite.visible() {
                        sprite.draw(ctx);
                    }
                }
            }
            scene.after_draw(ctx);
        }
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

        if let Some(scene) = self.current_scene() {
            let focused = scene
                .sprites()
                .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
                .unwrap_or(false);
            for t in scene.event(ctx, event, focused) {
                self.transit(ctx, t);
            }
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
