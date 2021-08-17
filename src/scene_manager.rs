use engine::{EngineContext, WindowMode};
use scenes::{EmptyScreen, MainMenu, Scene, SceneT, Settings};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod, Scancode};
use sdl2::mouse::MouseButton;
use sprite::{Button, ButtonState, Clickable, Sprite};
use std::collections::HashMap;

fn process_mouse_motion(mouse_pos: (i32, i32), sprites: &mut Vec<Sprite>) {
    fn process_hovered<B>(mouse_pos: (i32, i32), button: &mut B)
    where
        B: Clickable,
    {
        if button.state() != ButtonState::Disabled {
            let collide = button.rect().contains_point(mouse_pos);
            if collide && button.state() == ButtonState::Default {
                button.change_state(ButtonState::Hovered);
            } else if !collide && button.state() == ButtonState::Hovered {
                button.change_state(ButtonState::Default);
            }
        }
    }

    sprites.iter_mut().for_each(|sprite| match sprite {
        Sprite::Button(button) => {
            process_hovered(mouse_pos, button);
        }
        Sprite::RadioButton(button) => {
            process_hovered(mouse_pos, button);
        }
        _ => {}
    });
}

fn process_mouse_button_down(mouse_pos: (i32, i32), sprites: &mut Vec<Sprite>) {
    fn process_pressed<B>(mouse_pos: (i32, i32), button: &mut B)
    where
        B: Clickable,
    {
        if button.state() != ButtonState::Disabled {
            let collide = button.rect().contains_point(mouse_pos);
            if collide {
                button.change_state(ButtonState::Pressed);
            }
        }
    }
    sprites.iter_mut().for_each(|sprite| match sprite {
        Sprite::Button(button) => {
            process_pressed(mouse_pos, button);
        }
        Sprite::RadioButton(button) => {
            process_pressed(mouse_pos, button);
        }
        _ => {}
    });
}

fn process_mouse_button_up(mouse_pos: (i32, i32), sprites: &mut Vec<Sprite>) -> Option<String> {
    let mut clicked: Option<String> = None;
    let mut radio_set: Option<String> = None;
    for sprite in sprites.iter_mut() {
        match sprite {
            Sprite::Button(button) => {
                let collides = button.rect().contains_point(mouse_pos);
                match (collides, button.state) {
                    (true, ButtonState::Pressed) => {
                        button.state = ButtonState::Hovered;
                        // println!("clicked {}!", button.id);
                        clicked = Some(button.id.clone());
                    }
                    (false, ButtonState::Pressed) => {
                        button.change_state(ButtonState::Default);
                    }
                    _ => {}
                }
            }
            Sprite::RadioButton(button) => {
                let collides = button.rect().contains_point(mouse_pos);
                if collides && button.state == ButtonState::Pressed {
                    clicked = Some(button.id.clone());
                    // println!("clicked {}!", button.id);
                    radio_set = Some(button.radio_set.clone());
                }
            }
            _ => {}
        }
    }
    if let Some(radio_set) = radio_set {
        for sprite in sprites.iter_mut() {
            if let Sprite::RadioButton(other) = sprite {
                if other.radio_set == radio_set
                    && other.id != *clicked.as_ref().unwrap()
                    && other.state == ButtonState::Pressed
                {
                    other.state = ButtonState::Default;
                }
            }
        }
    }
    clicked
}

fn process_key_down(
    scancode: Scancode,
    keymod: Mod,
    focused_input: Option<usize>,
    sprites: &mut Vec<Sprite>,
) {
    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
    let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    if !shift && !alt && !ctrl {
        for (i, sprite) in sprites.iter_mut().enumerate() {
            if let Sprite::Button(button) = sprite {
                if button.key == scancode
                    || (focused_input.is_some() && focused_input.unwrap() == i)
                {
                    button.state = ButtonState::Pressed;
                }
            }
        }
    }
}

fn process_key_up(
    scancode: Scancode,
    keymod: Mod,
    focused_input: Option<usize>,
    sprites: &mut Vec<Sprite>,
) -> Option<String> {
    let mut clicked: Option<String> = None;
    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
    let alt = keymod.intersects(Mod::LALTMOD | Mod::RALTMOD);
    let ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    if !shift && !alt && !ctrl {
        for (i, sprite) in sprites.iter_mut().enumerate() {
            if let Sprite::Button(button) = sprite {
                if (button.key == scancode
                    || (focused_input.is_some() && focused_input.unwrap() == i))
                    && button.state == ButtonState::Pressed
                {
                    button.state = ButtonState::Default;
                    // println!("clicked {}!", button.id);
                    clicked = Some(button.id.clone());
                }
            }
        }
    }
    clicked
}

pub enum CallResult {
    DoNothing,
    SystemExit,
    ChangeScene(String),
    ChangeWindowMode(WindowMode),
}

pub struct SceneManager {
    scenes: HashMap<String, Scene>,
    current_scene: String,
    sprites: Vec<Sprite>,
    focused_input: Option<usize>,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: HashMap::with_capacity(5),
            current_scene: "main_menu".to_ascii_lowercase(),
            sprites: vec![],
            focused_input: None,
        }
    }

    pub fn on_open(&mut self, context: &mut EngineContext) {
        context.canvas.clear();
        self.current_scene().on_open(context);
        self.sprites = self.current_scene().create_sprites(context);
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
                "settings" => Settings {}.into(),
                _ => panic!("Unknown scene id: {}!", scene_id),
            };
            self.scenes.insert(scene_id.to_ascii_lowercase(), scene);
        }
        self.scenes.get_mut(scene_id).unwrap()
    }

    pub fn on_update(&mut self, context: &mut EngineContext, elapsed_time: f64) {
        if !self.sprites.is_empty() {
            context.draw_sprites(&self.sprites).ok();
        }
        self.current_scene().on_update(context, elapsed_time);
    }

    pub fn on_resize(&mut self, context: &mut EngineContext) {
        self.sprites = self.current_scene().create_sprites(context);
        self.current_scene().on_resize(context);
    }

    pub fn focused_input(&mut self) -> Option<&mut Button> {
        if !self.sprites.is_empty() {
            if let Some(f) = self.focused_input {
                return match self.sprites.get_mut(f).unwrap() {
                    Sprite::Button(button) => Some(button),
                    _ => None,
                };
            }
        }
        None
    }

    fn next_input(&mut self, forward: bool) {
        let buttons_len = self.sprites.len();
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
                process_mouse_motion((*x, *y), &mut self.sprites);
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                process_mouse_button_down((*x, *y), &mut self.sprites);
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some(clicked) = process_mouse_button_up((*x, *y), &mut self.sprites) {
                    if let Some(result) = self.current_scene().button_click(clicked.as_str()) {
                        return result;
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
                if !self.sprites.is_empty() {
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
                process_key_down(*scancode, *keymod, self.focused_input, &mut self.sprites);
            }
            Event::KeyUp {
                scancode: Some(scancode),
                keymod,
                ..
            } => {
                if let Some(clicked) =
                    process_key_up(*scancode, *keymod, self.focused_input, &mut self.sprites)
                {
                    self.focused_input = None;
                    if let Some(result) = self.current_scene().button_click(clicked.as_str()) {
                        return result;
                    }
                }
            }
            _ => {}
        }
        self.current_scene().call(context, event)
    }
}
