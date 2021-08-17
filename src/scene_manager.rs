use engine::{EngineContext, WindowMode};
use scenes::{CreateWorld, EmptyScreen, MainMenu, Scene, SceneT, Settings};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod, Scancode};
use sdl2::mouse::MouseButton;
use sprite::{Clickable, ClickableState, Sprite};
use std::collections::HashMap;

fn process_mouse_motion(mouse_pos: (i32, i32), sprites: &mut Vec<Sprite>) {
    fn process_hovered<B>(mouse_pos: (i32, i32), button: &mut B)
    where
        B: Clickable,
    {
        if button.state() != ClickableState::Disabled {
            let collide = button.rect().contains_point(mouse_pos);
            if collide && button.state() == ClickableState::Default {
                button.change_state(ClickableState::Hovered);
            } else if !collide && button.state() == ClickableState::Hovered {
                button.change_state(ClickableState::Default);
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
        Sprite::TextInput(input) => {
            process_hovered(mouse_pos, input);
        }
        _ => {}
    });
}

fn process_mouse_button_down(mouse_pos: (i32, i32), sprites: &mut Vec<Sprite>) {
    fn process_pressed<B>(mouse_pos: (i32, i32), button: &mut B)
    where
        B: Clickable,
    {
        if button.state() != ClickableState::Disabled {
            let collide = button.rect().contains_point(mouse_pos);
            if collide {
                button.change_state(ClickableState::Pressed);
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
        Sprite::TextInput(input) => {
            process_pressed(mouse_pos, input);
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
                    (true, ClickableState::Pressed) => {
                        button.state = ClickableState::Hovered;
                        // println!("clicked {}!", button.id);
                        clicked = Some(button.id.clone());
                    }
                    (false, ClickableState::Pressed) => {
                        button.change_state(ClickableState::Default);
                    }
                    _ => {}
                }
            }
            Sprite::RadioButton(button) => {
                let collides = button.rect().contains_point(mouse_pos);
                if collides && button.state == ClickableState::Pressed {
                    clicked = Some(button.id.clone());
                    // println!("clicked {}!", button.id);
                    radio_set = Some(button.radio_set.clone());
                }
            }
            Sprite::TextInput(input) => {
                let collides = input.rect().contains_point(mouse_pos);
                if collides && input.state == ClickableState::Pressed {
                    clicked = Some(input.id.clone());
                    // println!("clicked {}!", input.id);
                    input.state = ClickableState::Focused;
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
                    && other.state == ClickableState::Pressed
                {
                    other.state = ClickableState::Default;
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
                    button.state = ClickableState::Pressed;
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
                    && button.state == ClickableState::Pressed
                {
                    button.state = ClickableState::Default;
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

pub struct SceneData {
    pub sprites: Vec<Sprite>,
    pub focused_input: Option<usize>,
}

pub struct SceneManager {
    scenes: HashMap<String, Scene>,
    pub current_scene: String,
    pub scene_data: SceneData,
}

impl SceneManager {
    pub fn new() -> SceneManager {
        SceneManager {
            scenes: HashMap::with_capacity(5),
            current_scene: "main_menu".to_ascii_lowercase(),
            scene_data: SceneData {
                sprites: vec![],
                focused_input: None,
            },
        }
    }

    pub fn on_open(&mut self, context: &mut EngineContext) {
        context.canvas.clear();
        self.current_scene().on_open(context);
        self.scene_data.sprites = self.current_scene().create_sprites(context);
        self.scene_data.focused_input = None;
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
                "create_world" => CreateWorld {}.into(),
                "settings" => Settings {}.into(),
                _ => panic!("Unknown scene id: {}!", scene_id),
            };
            self.scenes.insert(scene_id.to_ascii_lowercase(), scene);
        }
        self.scenes.get_mut(scene_id).unwrap()
    }

    pub fn on_update(&mut self, context: &mut EngineContext, elapsed_time: f64) {
        if let Some(f) = self.scene_data.focused_input {
            if let Sprite::TextInput(input) = self.scene_data.sprites.get_mut(f).unwrap() {
                let ticks = (elapsed_time / 0.016).round() as u16;
                input.blink_elapsed += ticks;
                // println!("blink {}", input.blink);
                if input.blink_elapsed >= 30 {
                    input.blink_elapsed = 0;
                    input.blink = !input.blink;
                }
            }
        }
        if !self.scene_data.sprites.is_empty() {
            context.draw_sprites(&self.scene_data.sprites).ok();
        }
        self.current_scene().on_update(context, elapsed_time);
    }

    pub fn on_resize(&mut self, context: &mut EngineContext) {
        self.scene_data.sprites = self.current_scene().create_sprites(context);
        self.current_scene().on_resize(context);
    }

    pub fn focused_input(&mut self) -> Option<&mut dyn Clickable> {
        if !self.scene_data.sprites.is_empty() {
            if let Some(f) = self.scene_data.focused_input {
                return match self.scene_data.sprites.get_mut(f).unwrap() {
                    Sprite::Button(button) => Some(button),
                    // Sprite::RadioButton(button) => Some(button),
                    Sprite::TextInput(input) => Some(input),
                    _ => None,
                };
            }
        }
        None
    }

    fn next_input(&mut self, forward: bool) {
        let buttons_len = self.scene_data.sprites.len();
        if self.scene_data.focused_input.is_none() {
            let f = if forward { 0 } else { buttons_len - 1 };
            self.scene_data.focused_input = Some(f);
        } else {
            let mut f = self.scene_data.focused_input.unwrap();
            if forward {
                f = if f < buttons_len - 1 { f + 1 } else { 0 };
            } else {
                f = if f > 0 { f - 1 } else { buttons_len - 1 };
            }
            self.scene_data.focused_input = Some(f);
        }
    }

    pub fn call(&mut self, context: &mut EngineContext, event: &Event) -> CallResult {
        match event {
            Event::MouseMotion { x, y, .. } => {
                process_mouse_motion((*x, *y), &mut self.scene_data.sprites);
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                process_mouse_button_down((*x, *y), &mut self.scene_data.sprites);
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                x,
                y,
                ..
            } => {
                if let Some(clicked) =
                    process_mouse_button_up((*x, *y), &mut self.scene_data.sprites)
                {
                    for (i, sprite) in self.scene_data.sprites.iter().enumerate() {
                        if let Sprite::TextInput(input) = sprite {
                            if input.id == clicked {
                                self.scene_data.focused_input = Some(i);
                                break;
                            }
                        }
                    }
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
                if !self.scene_data.sprites.is_empty() {
                    let shift = keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD);
                    if self.focused_input().is_some() {
                        self.focused_input()
                            .unwrap()
                            .change_state(ClickableState::Default);
                    }
                    self.next_input(!shift);
                    let start = self.scene_data.focused_input.unwrap();
                    while self.focused_input().is_none()
                        || self.focused_input().unwrap().state() == ClickableState::Disabled
                    {
                        self.next_input(!shift);
                        if let Some(f) = self.scene_data.focused_input {
                            if f == start {
                                // we looping
                                self.scene_data.focused_input = None;
                                break;
                            }
                        }
                    }
                    if let Some(input) = self.focused_input() {
                        input.change_state(ClickableState::Focused);
                    }
                }
            }
            Event::KeyDown {
                scancode: Some(Scancode::Escape),
                ..
            } => {
                if self.scene_data.focused_input.is_some() {
                    if let Some(input) = self.focused_input() {
                        input.change_state(ClickableState::Default);
                        self.scene_data.focused_input = None;
                        return CallResult::DoNothing;
                    }
                }
            }
            Event::KeyDown {
                scancode: Some(Scancode::Backspace),
                ..
            } => {
                if let Some(f) = self.scene_data.focused_input {
                    if let Sprite::TextInput(input) = self.scene_data.sprites.get_mut(f).unwrap() {
                        if !input.value.is_empty() {
                            input.value =
                                input.value.as_str()[0..input.value.len() - 1].to_string();
                        }
                    }
                }
            }
            Event::TextInput { text, .. } => {
                if let Some(f) = self.scene_data.focused_input {
                    if let Sprite::TextInput(input) = self.scene_data.sprites.get_mut(f).unwrap() {
                        input.value.push_str(text.as_str());
                    }
                }
            }
            Event::KeyDown {
                scancode: Some(scancode),
                keymod,
                ..
            } => {
                process_key_down(
                    *scancode,
                    *keymod,
                    self.scene_data.focused_input,
                    &mut self.scene_data.sprites,
                );
            }
            Event::KeyUp {
                scancode: Some(scancode),
                keymod,
                ..
            } => {
                if let Some(clicked) = process_key_up(
                    *scancode,
                    *keymod,
                    self.scene_data.focused_input,
                    &mut self.scene_data.sprites,
                ) {
                    self.scene_data.focused_input = None;
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
