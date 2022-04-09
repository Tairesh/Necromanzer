use assets::Assets;
use colors::Colors;
use scenes;
use scenes::game_scene::GameScene;
use scenes::scene::Scene;
use scenes::transition::Transition;
use settings::game::GameSettings;
use sprites::label::Label;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Stringify};
use tetra::input::Key;
use tetra::{window, Context, Event, State};

pub struct App {
    pub assets: Assets,
    pub settings: GameSettings,
    scenes: Vec<Box<dyn Scene>>,
    fps_counter: Label,
}

impl App {
    pub fn new(ctx: &mut Context, settings: GameSettings) -> tetra::Result<Self> {
        let assets = Assets::load(ctx)?;
        let fps_counter = Label::new(
            "00",
            assets.fonts.default.clone(),
            Colors::BLACK,
            Position::by_right_top(-10.0, 10.0),
        );
        let mut app = Self {
            settings,
            assets,
            scenes: vec![],
            fps_counter,
        };
        app.push_scene(ctx, GameScene::MainMenu);
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
            scenes::scene::reposition_all_sprites(scene, ctx, window_size);
            scene.on_resize(ctx, window_size);
            self.fps_counter.positionate(ctx, window_size);
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

    fn exec_transitions(&mut self, ctx: &mut Context, transitions: Vec<Transition>) {
        for transition in transitions {
            self.transit(ctx, transition);
        }
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
        // TODO: find a way to optimize this shit
        if let Some(scene) = self.current_scene() {
            let mut transitions = scene.update(ctx);
            let focused = scenes::scene::is_there_focused_sprite(scene);
            if let Some(sprites) = scene.sprites() {
                // creating same big useless vec of Rects EVERY frame
                let mut blocked = Vec::with_capacity(sprites.len());
                for sprite in sprites.iter().rev() {
                    let mut sprite = sprite.borrow_mut();
                    if let Some(transition) = sprite.update(ctx, focused, &blocked) {
                        transitions.push(transition);
                    }
                    if sprite.visible() && sprite.block_mouse() {
                        blocked.push(sprite.rect());
                    }
                }
            }
            self.exec_transitions(ctx, transitions);
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
            let fps = (tetra::time::get_fps(ctx).round() as u8).to_string();
            if !self.fps_counter.value().eq(&fps) {
                self.fps_counter.set_value(fps);
            }
            self.fps_counter.draw(ctx);
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
            let transitions = scene.event(ctx, event);
            self.exec_transitions(ctx, transitions);
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
