use engine::EngineContext;
use scene_manager::CallResult;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sprite::{Button, ButtonState, Image, Label, LabelFont, SceneSprites};
use {colors, VERSION};

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult;
    fn button_click(&mut self, button_id: &str) -> Option<CallResult>;
    fn on_open(&mut self, context: &mut EngineContext);
    fn create_sprites(&mut self, context: &mut EngineContext) -> Option<SceneSprites>;
    fn on_resize(&mut self, context: &mut EngineContext);
    fn on_update(&mut self, context: &mut EngineContext, elapsed_dime: f64);
}

#[derive(Hash, Eq, PartialEq)]
pub struct MainMenu;
impl SceneT for MainMenu {
    fn call(&mut self, _context: &mut EngineContext, _event: &Event) -> CallResult {
        CallResult::DoNothing
    }

    fn button_click(&mut self, button_id: &str) -> Option<CallResult> {
        match button_id {
            "exit" => Some(CallResult::SystemExit),
            "create_world" => Some(CallResult::ChangeScene("empty_screen".to_ascii_lowercase())),
            "settings" => Some(CallResult::ChangeScene("settings".to_ascii_lowercase())),
            _ => None,
        }
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, context: &mut EngineContext) -> Option<SceneSprites> {
        let mut sprites = SceneSprites::new();
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        sprites.add_sprite(
            Image {
                path: "res/img/bg.jpg".to_string(),
                position: (
                    screen_center.0 - bg_size.0 as i32 / 2,
                    screen_center.1 - bg_size.1 as i32 / 2,
                ),
            }
            .into(),
        );
        let logo_size = context.sprite_manager.image_size("res/img/logo.png");
        sprites.add_sprite(
            Image {
                path: "res/img/logo.png".to_string(),
                position: (screen_center.0 - logo_size.0 as i32 / 2, 10),
            }
            .into(),
        );
        let version_size = context
            .sprite_manager
            .text_size(&*VERSION, LabelFont::Default);
        sprites.add_sprite(
            Label {
                text: (*VERSION).to_string(),
                font: LabelFont::Default,
                color: None,
                position: (
                    screen_center.0 + logo_size.0 as i32 / 2 - version_size.0 as i32 - 20,
                    logo_size.1 as i32 + 10,
                ),
            }
            .into(),
        );
        let button_size = |text: &str| {
            (
                context.sprite_manager.text_size(text, LabelFont::Default).0 + 20,
                30,
            )
        };
        let load_button_text = "[l] Load world";
        let load_button_size = button_size(load_button_text);
        sprites.add_sprite(
            Button {
                id: "load_world".to_ascii_lowercase(),
                key: Scancode::L,
                text: load_button_text.to_string(),
                size: load_button_size,
                position: (screen_center.0 - load_button_size.0 as i32 / 2, 300),
                state: ButtonState::Disabled,
            }
            .into(),
        );
        let create_button_text = "[c] Create new world";
        let create_button_size = button_size(create_button_text);
        sprites.add_sprite(
            Button {
                id: "create_world".to_ascii_lowercase(),
                key: Scancode::C,
                text: create_button_text.to_string(),
                size: create_button_size,
                position: (screen_center.0 - create_button_size.0 as i32 / 2, 340),
                state: ButtonState::Default,
            }
            .into(),
        );
        let settings_button_text = "[s] Settings";
        let settings_button_size = button_size(settings_button_text);
        sprites.add_sprite(
            Button {
                id: "settings".to_ascii_lowercase(),
                key: Scancode::S,
                text: settings_button_text.to_string(),
                size: settings_button_size,
                position: (screen_center.0 - settings_button_size.0 as i32 / 2, 380),
                state: ButtonState::Default,
            }
            .into(),
        );
        let exit_button_text = "[x] Exit";
        let exit_button_size = button_size(exit_button_text);
        sprites.add_sprite(
            Button {
                id: "exit".to_ascii_lowercase(),
                key: Scancode::X,
                text: exit_button_text.to_string(),
                size: exit_button_size,
                position: (screen_center.0 - exit_button_size.0 as i32 / 2, 420),
                state: ButtonState::Default,
            }
            .into(),
        );

        Some(sprites)
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, _context: &mut EngineContext, _elapsed_dime: f64) {}
}

#[derive(Hash, Eq, PartialEq)]
pub struct EmptyScreen;
impl SceneT for EmptyScreen {
    fn call(&mut self, _context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } => CallResult::ChangeScene("main_menu".to_ascii_lowercase()),
            _ => CallResult::DoNothing,
        }
    }

    fn button_click(&mut self, _button_id: &str) -> Option<CallResult> {
        None
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, _context: &mut EngineContext) -> Option<SceneSprites> {
        None
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, context: &mut EngineContext, _elapsed_dime: f64) {
        context
            .canvas
            .set_draw_color(colors::rgb(colors::DARK_GREEN));
        context.canvas.clear();
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Settings;
impl SceneT for Settings {
    fn call(&mut self, _context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::X1,
                ..
            }
            | Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } => CallResult::ChangeScene("main_menu".to_ascii_lowercase()),
            _ => CallResult::DoNothing,
        }
    }

    fn button_click(&mut self, _button_id: &str) -> Option<CallResult> {
        None
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, context: &mut EngineContext) -> Option<SceneSprites> {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let mut sprites = SceneSprites::new();
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        sprites.add_sprite(
            Image {
                path: "res/img/bg.jpg".to_string(),
                position: (
                    screen_center.0 - bg_size.0 as i32 / 2,
                    screen_center.1 - bg_size.1 as i32 / 2,
                ),
            }
            .into(),
        );
        let title_size = context
            .sprite_manager
            .text_size("Settings", LabelFont::Title);
        sprites.add_sprite(
            Label {
                text: "Settings".to_string(),
                font: LabelFont::Title,
                color: Some(colors::rgb(colors::DARK_GREEN)),
                position: (screen_center.0 - title_size.0 as i32 / 2, 30),
            }
            .into(),
        );
        Some(sprites)
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, _context: &mut EngineContext, _elapsed_dime: f64) {}
}

#[enum_dispatch::enum_dispatch(SceneT)]
#[derive(Hash, Eq, PartialEq)]
pub enum Scene {
    MainMenu,
    EmptyScreen,
    Settings,
}
