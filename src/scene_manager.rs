use engine::EngineContext;
use scenes::{EmptyScreen, MainMenu, Scene, SceneT};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod, Scancode};
use sdl2::mouse::MouseButton;
use sprite::{Button, ButtonState, ImgSprite, TextSprite};
use std::collections::HashMap;

fn collide(mouse_pos: (i32, i32), button: &Button) -> bool {
    let (left, top) = (button.position.0, button.position.1);
    let (right, bottom) = (left + button.size.0 as i32, top + button.size.1 as i32);
    let (x, y) = mouse_pos;
    left <= x && x <= right && top <= y && y <= bottom
}

fn process_mouse_motion(mouse_pos: (i32, i32), data: &mut SpritesData) {
    data.buttons.iter_mut().for_each(|button| {
        if button.state != ButtonState::Disabled {
            let collide = collide(mouse_pos, button);
            if collide && button.state == ButtonState::Default {
                button.state = ButtonState::Hovered;
            } else if !collide && button.state == ButtonState::Hovered {
                button.state = ButtonState::Default;
            }
        }
    });
}

fn process_mouse_button_down(mouse_pos: (i32, i32), data: &mut SpritesData) {
    data.buttons.iter_mut().for_each(|button| {
        if button.state != ButtonState::Disabled {
            let collide = collide(mouse_pos, button);
            if collide {
                button.state = ButtonState::Pressed;
            }
        }
    });
}

fn process_mouse_button_up(mouse_pos: (i32, i32), data: &mut SpritesData) -> Option<String> {
    let mut clicked: Option<String> = None;
    for button in data.buttons.iter_mut() {
        let collides = collide(mouse_pos, button);
        if collides && button.state == ButtonState::Pressed {
            button.state = ButtonState::Hovered;
            // println!("clicked {}!", button.id);
            clicked = Some(button.id.clone());
        } else if !collides && button.state == ButtonState::Pressed {
            button.state = ButtonState::Default;
        }
    }
    clicked
}

fn process_key_down(
    scancode: Scancode,
    keymod: Mod,
    focused_input: Option<usize>,
    data: &mut SpritesData,
) {
    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
    let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    if !shift && !alt && !ctrl {
        for (i, button) in data.buttons.iter_mut().enumerate() {
            if button.key == scancode || (focused_input.is_some() && focused_input.unwrap() == i) {
                button.state = ButtonState::Pressed;
            }
        }
    }
}

fn process_key_up(
    scancode: Scancode,
    keymod: Mod,
    focused_input: Option<usize>,
    data: &mut SpritesData,
) -> Option<String> {
    let mut clicked: Option<String> = None;
    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
    let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    if !shift && !alt && !ctrl {
        for (i, button) in data.buttons.iter_mut().enumerate() {
            if (button.key == scancode || (focused_input.is_some() && focused_input.unwrap() == i))
                && button.state == ButtonState::Pressed
            {
                button.state = ButtonState::Default;
                // println!("clicked {}!", button.id);
                clicked = Some(button.id.clone());
            }
        }
    }
    clicked
}

pub enum CallResult {
    DoNothing,
    SystemExit,
    ChangeScene(String),
}

#[derive(Hash, Eq, PartialEq)]
pub struct SpritesData {
    pub img_sprites: Vec<ImgSprite>,
    pub text_sprites: Vec<TextSprite>,
    pub buttons: Vec<Button>,
}

pub struct SceneManager {
    scenes: HashMap<String, Scene>,
    current_scene: String,
    sprites_data: Option<SpritesData>,
    focused_input: Option<usize>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: HashMap::with_capacity(5),
            current_scene: "main_menu".to_ascii_lowercase(),
            sprites_data: None,
            focused_input: None,
        }
    }

    pub fn focused_input(&mut self) -> Option<&mut Button> {
        if let (Some(sprites), Some(f)) = (self.sprites_data.as_mut(), self.focused_input) {
            return sprites.buttons.get_mut(f);
        }
        None
    }

    pub fn on_open(&mut self, context: &mut EngineContext) {
        self.current_scene().on_open(context);
        self.sprites_data = self.current_scene().create_sprites(context);
        self.focused_input = None;
    }

    pub fn change_scene(&mut self, context: &mut EngineContext, new_scene_id: &str) {
        self.current_scene = new_scene_id.to_ascii_lowercase();
        self.on_open(context);
    }

    pub fn current_scene(&mut self) -> &mut Scene {
        let scene_id = self.current_scene.as_str();
        if !self.scenes.contains_key(scene_id) {
            let scene: Scene = match scene_id {
                "main_menu" => MainMenu {}.into(),
                "empty_screen" => EmptyScreen {}.into(),
                _ => panic!(format!("Unknown scene id: {}!", scene_id)),
            };
            self.scenes.insert(scene_id.to_ascii_lowercase(), scene);
        }
        self.scenes.get_mut(scene_id).unwrap()
    }

    pub fn on_update(&mut self, context: &mut EngineContext, elapsed_time: f64) {
        if let Some(sprites) = self.sprites_data.as_ref() {
            context.draw_sprites(sprites).ok();
        }
        self.current_scene().on_update(context, elapsed_time);
    }

    pub fn on_resize(&mut self, context: &mut EngineContext) {
        self.sprites_data = self.current_scene().create_sprites(context);
        self.current_scene().on_resize(context);
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

    pub fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseMotion { x, y, .. } => {
                if let Some(sprites) = self.sprites_data.as_mut() {
                    process_mouse_motion((*x, *y), sprites);
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some(sprites) = self.sprites_data.as_mut() {
                    process_mouse_button_down((*x, *y), sprites);
                }
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some(sprites) = self.sprites_data.as_mut() {
                    if let Some(clicked) = process_mouse_button_up((*x, *y), sprites) {
                        if let Some(result) = self.current_scene().button_click(clicked.as_str()) {
                            return result;
                        }
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::F2),
                ..
            } => {
                context.set_show_fps(!context.fps_counter.show_fps);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Tab),
                keymod,
                ..
            } => {
                if self.sprites_data.is_some() {
                    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                    if self.focused_input.is_some() {
                        self.focused_input().unwrap().state = ButtonState::Default;
                    }
                    self.next_input(!shift);
                    while self.focused_input().unwrap().state == ButtonState::Disabled {
                        self.next_input(!shift);
                    }
                    self.focused_input().unwrap().state = ButtonState::Focused;
                }
            }
            Event::KeyDown {
                scancode: Some(scancode),
                keymod,
                ..
            } => {
                if let Some(sprites) = self.sprites_data.as_mut() {
                    process_key_down(*scancode, *keymod, self.focused_input, sprites);
                }
            }
            Event::KeyUp {
                scancode: Some(scancode),
                keymod,
                ..
            } => {
                if let Some(sprites) = self.sprites_data.as_mut() {
                    if let Some(clicked) =
                        process_key_up(*scancode, *keymod, self.focused_input, sprites)
                    {
                        self.focused_input = None;
                        if let Some(result) = self.current_scene().button_click(clicked.as_str()) {
                            return result;
                        }
                    }
                }
            }
            _ => {}
        }
        self.current_scene().call(context, event)
    }
}
