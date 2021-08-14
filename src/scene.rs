use engine::EngineContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sprite::{ImgSprite, TextSprite};
use VERSION;

pub enum CallResult {
    DoNothing,
    SystemExit,
    ChangeScene(Scene),
}

#[derive(Hash, Eq, PartialEq)]
pub struct SpritesData {
    pub img_sprites: Vec<ImgSprite>,
    pub text_sprites: Vec<TextSprite>,
}

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult;
    fn on_open(&mut self, context: &mut EngineContext);
    fn on_resize(&mut self, context: &mut EngineContext);
    fn on_update(&mut self, context: &mut EngineContext, elapsed_dime: f64) -> CallResult;
}

#[derive(Hash, Eq, PartialEq)]
pub struct MainMenu {
    sprites_data: Option<SpritesData>,
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu { sprites_data: None }
    }

    fn positionate_sprites(&mut self, context: &mut EngineContext) {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let bg = context.textures.load_image("res/img/bg.jpg");
        let bg_position = Rect::new(
            screen_center.0 - bg.query().width as i32 / 2,
            screen_center.1 - bg.query().height as i32 / 2,
            bg.query().width,
            bg.query().height,
        );
        let logo = context.textures.load_image("res/img/logo.png");
        let logo_position = Rect::new(
            screen_center.0 - logo.query().width as i32 / 2,
            10,
            logo.query().width,
            logo.query().height,
        );
        let version_label = context.textures.render_text(&*VERSION, None);
        let version_position = Rect::new(
            logo_position.right() - version_label.query().width as i32 - 20,
            logo_position.bottom(),
            version_label.query().width,
            version_label.query().height,
        );
        self.sprites_data = Some(SpritesData {
            img_sprites: vec![
                ImgSprite {
                    path: "res/img/bg.jpg".to_string(),
                    position: bg_position,
                },
                ImgSprite {
                    path: "res/img/logo.png".to_string(),
                    position: logo_position,
                },
            ],
            text_sprites: vec![TextSprite {
                text: (*VERSION).to_string(),
                color: None,
                position: version_position,
            }],
        });
    }
}
impl SceneT for MainMenu {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => CallResult::ChangeScene(EmptyScreen {}.into()),
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => CallResult::SystemExit,
            Event::KeyDown {
                keycode: Some(Keycode::F11),
                ..
            } => {
                context.set_show_fps(!context.fps_counter.show_fps);
                CallResult::DoNothing
            }
            _ => CallResult::DoNothing,
        }
    }

    fn on_open(&mut self, context: &mut EngineContext) {
        if self.sprites_data.is_none() {
            self.positionate_sprites(context);
        }
    }

    fn on_resize(&mut self, context: &mut EngineContext) {
        self.positionate_sprites(context);
    }

    fn on_update(&mut self, context: &mut EngineContext, _elapsed_dime: f64) -> CallResult {
        context
            .draw_sprites(self.sprites_data.as_ref().unwrap())
            .ok();
        CallResult::DoNothing
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct EmptyScreen;
impl SceneT for EmptyScreen {
    fn call(&mut self, _context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => CallResult::ChangeScene(MainMenu::new().into()),
            _ => CallResult::DoNothing,
        }
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, context: &mut EngineContext, _elapsed_dime: f64) -> CallResult {
        context.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.canvas.clear();
        CallResult::DoNothing
    }
}

#[enum_dispatch::enum_dispatch(SceneT)]
#[derive(Hash, Eq, PartialEq)]
pub enum Scene {
    MainMenu,
    EmptyScreen,
}
