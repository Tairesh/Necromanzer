use engine::{EngineContext, WindowMode};
use scene_manager::CallResult;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sprite::{Button, ClickableState, Image, Label, LabelFont, RadioButton, Sprite, TextInput};
use {colors, VERSION};

fn bg_img(position: (i32, i32)) -> Image {
    Image {
        path: "res/img/bg.jpg".to_string(),
        position,
    }
}

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult;
    fn button_click(&mut self, button_id: &str) -> Option<CallResult>;
    fn on_open(&mut self, context: &mut EngineContext);
    fn create_sprites(&mut self, context: &mut EngineContext) -> Vec<Sprite>;
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
            "create_world" => Some(CallResult::ChangeScene("create_world".to_ascii_lowercase())),
            "settings" => Some(CallResult::ChangeScene("settings".to_ascii_lowercase())),
            _ => None,
        }
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, context: &mut EngineContext) -> Vec<Sprite> {
        let mut sprites = Vec::with_capacity(8);
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        sprites.push(
            bg_img((
                screen_center.0 - bg_size.0 as i32 / 2,
                screen_center.1 - bg_size.1 as i32 / 2,
            ))
            .into(),
        );
        let logo_size = context.sprite_manager.image_size("res/img/logo.png");
        sprites.push(
            Image {
                path: "res/img/logo.png".to_string(),
                position: (screen_center.0 - logo_size.0 as i32 / 2, 10),
            }
            .into(),
        );
        let version_size = context
            .sprite_manager
            .text_size(&*VERSION, LabelFont::Default);
        sprites.push(
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
        sprites.push(
            Button {
                id: "load_world".to_ascii_lowercase(),
                key: Scancode::L,
                text: load_button_text.to_string(),
                size: load_button_size,
                position: (screen_center.0 - load_button_size.0 as i32 / 2, 300),
                state: ClickableState::Disabled,
            }
            .into(),
        );
        let create_button_text = "[c] Create new world";
        let create_button_size = button_size(create_button_text);
        sprites.push(
            Button {
                id: "create_world".to_ascii_lowercase(),
                key: Scancode::C,
                text: create_button_text.to_string(),
                size: create_button_size,
                position: (screen_center.0 - create_button_size.0 as i32 / 2, 340),
                state: ClickableState::Default,
            }
            .into(),
        );
        let settings_button_text = "[s] Settings";
        let settings_button_size = button_size(settings_button_text);
        sprites.push(
            Button {
                id: "settings".to_ascii_lowercase(),
                key: Scancode::S,
                text: settings_button_text.to_string(),
                size: settings_button_size,
                position: (screen_center.0 - settings_button_size.0 as i32 / 2, 380),
                state: ClickableState::Default,
            }
            .into(),
        );
        let exit_button_text = "[x] Exit";
        let exit_button_size = button_size(exit_button_text);
        sprites.push(
            Button {
                id: "exit".to_ascii_lowercase(),
                key: Scancode::X,
                text: exit_button_text.to_string(),
                size: exit_button_size,
                position: (screen_center.0 - exit_button_size.0 as i32 / 2, 420),
                state: ClickableState::Default,
            }
            .into(),
        );

        sprites
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

    fn create_sprites(&mut self, _context: &mut EngineContext) -> Vec<Sprite> {
        vec![]
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

    fn button_click(&mut self, button_id: &str) -> Option<CallResult> {
        match button_id {
            "window_mode_fullscreen" => Some(CallResult::ChangeWindowMode(WindowMode::Fullscreen)),
            "window_mode_borderless" => Some(CallResult::ChangeWindowMode(WindowMode::Borderless)),
            "window_mode_default" => Some(CallResult::ChangeWindowMode(WindowMode::Window)),
            _ => None,
        }
    }

    fn on_open(&mut self, _context: &mut EngineContext) {}

    fn create_sprites(&mut self, context: &mut EngineContext) -> Vec<Sprite> {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let mut sprites = Vec::with_capacity(2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        sprites.push(
            bg_img((
                screen_center.0 - bg_size.0 as i32 / 2,
                screen_center.1 - bg_size.1 as i32 / 2,
            ))
            .into(),
        );
        let title_size = context
            .sprite_manager
            .text_size("Settings", LabelFont::Header1);
        sprites.push(
            Label {
                text: "Settings".to_string(),
                font: LabelFont::Header1,
                color: Some(colors::rgb(colors::DARK_GREEN)),
                position: (screen_center.0 - title_size.0 as i32 / 2, 10),
            }
            .into(),
        );
        let window_mode_label_text = "Window mode:";
        let window_mode_label_size = context
            .sprite_manager
            .text_size(window_mode_label_text, LabelFont::Header2);
        sprites.push(
            Label {
                text: window_mode_label_text.to_string(),
                font: LabelFont::Header2,
                color: None,
                position: (
                    screen_center.0 - 100 - window_mode_label_size.0 as i32 - 10,
                    100,
                ),
            }
            .into(),
        );
        let fullscreen_btn_text = "Fullscreen";
        let fullscreen_btn_width = context
            .sprite_manager
            .text_size(fullscreen_btn_text, LabelFont::Default)
            .0
            + 20;
        let mut fullscreen_btn = RadioButton {
            id: "window_mode_fullscreen".to_ascii_lowercase(),
            radio_set: "window_mode".to_ascii_lowercase(),
            text: fullscreen_btn_text.to_string(),
            size: (fullscreen_btn_width, 30),
            position: (
                screen_center.0 - 100,
                100 + window_mode_label_size.1 as i32 / 2 - 15,
            ),
            state: ClickableState::Default,
        };
        if context.window_mode == WindowMode::Fullscreen {
            fullscreen_btn.state = ClickableState::Pressed;
        }
        sprites.push(fullscreen_btn.into());
        let borderless_btn_text = "Fullscreen window";
        let borderless_btn_width = context
            .sprite_manager
            .text_size(borderless_btn_text, LabelFont::Default)
            .0
            + 20;
        let mut borderless_btn = RadioButton {
            id: "window_mode_borderless".to_ascii_lowercase(),
            radio_set: "window_mode".to_ascii_lowercase(),
            text: borderless_btn_text.to_string(),
            size: (borderless_btn_width, 30),
            position: (
                screen_center.0 - 100 + fullscreen_btn_width as i32 + 2,
                100 + window_mode_label_size.1 as i32 / 2 - 15,
            ),
            state: ClickableState::Default,
        };
        if context.window_mode == WindowMode::Borderless {
            borderless_btn.state = ClickableState::Pressed;
        }
        sprites.push(borderless_btn.into());
        let window_btn_text = "Window";
        let window_btn_width = context
            .sprite_manager
            .text_size(window_btn_text, LabelFont::Default)
            .0
            + 20;
        let mut window_btn = RadioButton {
            id: "window_mode_default".to_ascii_lowercase(),
            radio_set: "window_mode".to_ascii_lowercase(),
            text: window_btn_text.to_string(),
            size: (window_btn_width, 30),
            position: (
                screen_center.0 - 100
                    + fullscreen_btn_width as i32
                    + borderless_btn_width as i32
                    + 4,
                100 + window_mode_label_size.1 as i32 / 2 - 15,
            ),
            state: ClickableState::Default,
        };
        if context.window_mode == WindowMode::Window {
            window_btn.state = ClickableState::Pressed;
        }
        sprites.push(window_btn.into());
        sprites
    }

    fn on_resize(&mut self, _context: &mut EngineContext) {}

    fn on_update(&mut self, _context: &mut EngineContext, _elapsed_dime: f64) {}
}

#[derive(Hash, Eq, PartialEq)]
pub struct CreateWorld;
impl SceneT for CreateWorld {
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

    fn create_sprites(&mut self, context: &mut EngineContext) -> Vec<Sprite> {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let mut sprites = Vec::with_capacity(2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        sprites.push(
            bg_img((
                screen_center.0 - bg_size.0 as i32 / 2,
                screen_center.1 - bg_size.1 as i32 / 2,
            ))
            .into(),
        );
        let title = "Creating new world";
        let title_size = context.sprite_manager.text_size(title, LabelFont::Header1);
        sprites.push(
            Label {
                text: title.to_string(),
                font: LabelFont::Header1,
                color: Some(colors::rgb(colors::DARK_GREEN)),
                position: (screen_center.0 - title_size.0 as i32 / 2, 10),
            }
            .into(),
        );
        let name = "Name:";
        let name_size = context.sprite_manager.text_size(name, LabelFont::Header2);
        sprites.push(
            Label {
                text: name.to_string(),
                font: LabelFont::Header2,
                color: None,
                position: (screen_center.0 - 100 - name_size.0 as i32 - 10, 100),
            }
            .into(),
        );
        sprites.push(
            TextInput {
                id: "name".to_string(),
                value: "".to_string(),
                size: (300, 30),
                position: (screen_center.0 - 100, 100 + name_size.1 as i32 / 2 - 15),
                state: ClickableState::Default,
                blink: false,
                blink_elapsed: 30,
            }
            .into(),
        );
        let seed = "Seed:";
        let seed_size = context.sprite_manager.text_size(seed, LabelFont::Header2);
        sprites.push(
            Label {
                text: seed.to_string(),
                font: LabelFont::Header2,
                color: None,
                position: (screen_center.0 - 100 - seed_size.0 as i32 - 10, 150),
            }
            .into(),
        );
        sprites
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
    CreateWorld,
}
