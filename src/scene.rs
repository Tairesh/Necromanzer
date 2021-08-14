use engine::EngineContext;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod};
use sdl2::mouse::MouseButton;
use sprite::{Button, ButtonState, ImgSprite, TextSprite};
use {colors, VERSION};

fn collide(mouse_pos: (i32, i32), button: &Button) -> bool {
    let (left, top) = (button.position.0, button.position.1);
    let (right, bottom) = (left + button.size.0 as i32, top + button.size.1 as i32);
    let (x, y) = mouse_pos;
    left <= x && x <= right && top <= y && y <= bottom
}

pub enum CallResult {
    DoNothing,
    SystemExit,
    ChangeScene(Scene),
}

#[derive(Hash, Eq, PartialEq)]
pub struct SpritesData {
    pub img_sprites: Vec<ImgSprite>,
    pub text_sprites: Vec<TextSprite>,
    pub buttons: Vec<Button>,
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
    focused_input: Option<usize>,
}
impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            sprites_data: None,
            focused_input: None,
        }
    }

    fn positionate_sprites(&mut self, context: &mut EngineContext) {
        let (w, h) = context.canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);
        let bg_size = context.sprite_manager.image_size("res/img/bg.jpg");
        let logo_size = context.sprite_manager.image_size("res/img/logo.png");
        let version_size = context.sprite_manager.text_size(&*VERSION);
        let button_size = (220, 30);
        self.sprites_data = Some(SpritesData {
            img_sprites: vec![
                ImgSprite {
                    path: "res/img/bg.jpg".to_string(),
                    position: (
                        screen_center.0 - bg_size.0 as i32 / 2,
                        screen_center.1 - bg_size.1 as i32 / 2,
                    ),
                },
                ImgSprite {
                    path: "res/img/logo.png".to_string(),
                    position: (screen_center.0 - logo_size.0 as i32 / 2, 10),
                },
            ],
            text_sprites: vec![TextSprite {
                text: (*VERSION).to_string(),
                color: None,
                position: (
                    screen_center.0 + logo_size.0 as i32 / 2 - version_size.0 as i32 - 20,
                    logo_size.1 as i32 + 10,
                ),
            }],
            buttons: vec![
                Button {
                    id: "load_world".to_ascii_lowercase(),
                    key: Keycode::L,
                    text: "[l] Load world".to_string(),
                    size: button_size,
                    position: (screen_center.0 - button_size.0 as i32 / 2, 300),
                    state: ButtonState::Disabled,
                },
                Button {
                    id: "create_world".to_ascii_lowercase(),
                    key: Keycode::C,
                    text: "[c] Create new world".to_string(),
                    size: button_size,
                    position: (screen_center.0 - button_size.0 as i32 / 2, 340),
                    state: ButtonState::Default,
                },
                Button {
                    id: "settings".to_ascii_lowercase(),
                    key: Keycode::S,
                    text: "[s] Settings".to_string(),
                    size: button_size,
                    position: (screen_center.0 - button_size.0 as i32 / 2, 380),
                    state: ButtonState::Default,
                },
                Button {
                    id: "exit".to_ascii_lowercase(),
                    key: Keycode::X,
                    text: "[x] Exit".to_string(),
                    size: button_size,
                    position: (screen_center.0 - button_size.0 as i32 / 2, 420),
                    state: ButtonState::Default,
                },
            ],
        });
    }

    fn next_input(&mut self, forward: bool) {
        let buttons_len = self.sprites_data.as_ref().unwrap().buttons.len();
        if self.focused_input.is_none() {
            let f = if forward { 0 } else { buttons_len - 1 };
            self.focused_input = Some(f);
        } else {
            let mut f = self.focused_input.unwrap();
            if forward {
                f = if f < buttons_len - 1 { f + 1 } else { 0 };
            } else {
                f = if f > 0 { f - 1 } else { buttons_len - 1 };
            }
            self.focused_input = Some(f);
        }
    }

    fn button_click(button_id: &str) -> Option<CallResult> {
        match button_id {
            "exit" => Some(CallResult::SystemExit),
            "settings" => Some(CallResult::ChangeScene(EmptyScreen {}.into())),
            _ => None,
        }
    }
}
impl SceneT for MainMenu {
    fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.sprites_data
                    .as_mut()
                    .unwrap()
                    .buttons
                    .iter_mut()
                    .for_each(|button| {
                        if button.state != ButtonState::Disabled {
                            let collide = collide((*x, *y), button);
                            if collide && button.state == ButtonState::Default {
                                button.state = ButtonState::Hovered;
                            } else if !collide && button.state == ButtonState::Hovered {
                                button.state = ButtonState::Default;
                            }
                        }
                    });
                CallResult::DoNothing
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                self.sprites_data
                    .as_mut()
                    .unwrap()
                    .buttons
                    .iter_mut()
                    .for_each(|button| {
                        if button.state != ButtonState::Disabled {
                            let collide = collide((*x, *y), button);
                            if collide {
                                button.state = ButtonState::Pressed;
                            }
                        }
                    });
                CallResult::DoNothing
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                for button in self.sprites_data.as_mut().unwrap().buttons.iter_mut() {
                    if button.state != ButtonState::Disabled {
                        let collide = collide((*x, *y), button);
                        if collide && button.state == ButtonState::Pressed {
                            button.state = ButtonState::Hovered;
                            // println!("clicked {}!", button.id);
                            if let Some(result) = MainMenu::button_click(button.id.as_str()) {
                                return result;
                            }
                        }
                        button.state = ButtonState::Default;
                    }
                }
                CallResult::DoNothing
            }
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
            Event::KeyDown {
                keycode: Some(Keycode::Tab),
                keymod,
                ..
            } => {
                let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                if self.focused_input.is_some() {
                    let mut button = self
                        .sprites_data
                        .as_mut()
                        .unwrap()
                        .buttons
                        .get_mut(self.focused_input.unwrap())
                        .unwrap();
                    button.state = ButtonState::Default;
                }
                self.next_input(!shift);
                while self
                    .sprites_data
                    .as_ref()
                    .unwrap()
                    .buttons
                    .get(self.focused_input.unwrap())
                    .unwrap()
                    .state
                    == ButtonState::Disabled
                {
                    self.next_input(!shift);
                }
                let button = self
                    .sprites_data
                    .as_mut()
                    .unwrap()
                    .buttons
                    .get_mut(self.focused_input.unwrap())
                    .unwrap();
                button.state = ButtonState::Focused;

                CallResult::DoNothing
            }
            Event::KeyDown {
                keycode: Some(keycode),
                keymod,
                ..
            } => {
                let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
                let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
                if shift || alt || ctrl {
                    return CallResult::DoNothing;
                }
                for button in self.sprites_data.as_mut().unwrap().buttons.iter_mut() {
                    if button.key == *keycode {
                        button.state = ButtonState::Pressed;
                    }
                }
                CallResult::DoNothing
            }
            Event::KeyUp {
                keycode: Some(keycode),
                keymod,
                ..
            } => {
                let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
                let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
                if shift || alt || ctrl {
                    return CallResult::DoNothing;
                }
                for button in self.sprites_data.as_mut().unwrap().buttons.iter_mut() {
                    if button.key == *keycode && button.state == ButtonState::Pressed {
                        button.state = ButtonState::Default;
                        // println!("clicked {}!", button.id);
                        if let Some(result) = MainMenu::button_click(button.id.as_str()) {
                            return result;
                        }
                    }
                }
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
        context.canvas.set_draw_color(colors::rgb(colors::BLACK));
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
