use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::math::Rect;
use tetra::{input, Context, TetraVec2};

pub struct Button {
    id: String,
    keys: Vec<(Key, Option<KeyModifier>)>,
    text: Text,
    position: Position,
    assets: Rc<RefCell<Assets>>,
    rect: Option<Rect<f32, f32>>,
    is_pressed: bool,
    is_disabled: bool,
    is_hovered: bool,
    fixable: bool,
    dirty: bool,
    visible: bool,
}

impl Button {
    pub fn new(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        Button {
            id: id.to_string(),
            keys,
            text: Text::new(text, assets.borrow().default.clone()),
            position,
            assets: assets.clone(),
            rect: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            fixable: false,
            dirty: false,
            visible: true,
        }
    }

    pub fn fixed(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        state: bool,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Button {
        Button {
            id: id.to_string(),
            keys,
            text: Text::new(text, assets.borrow().default.clone()),
            position,
            assets: assets.clone(),
            rect: None,
            is_pressed: state,
            is_hovered: false,
            is_disabled: false,
            fixable: true,
            dirty: false,
            visible: true,
        }
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }
}

impl Draw for Button {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = TetraVec2::new(rect.x, rect.y);
        let bounds = self.text.get_bounds(ctx).unwrap();
        let assets = self.assets.borrow();

        let config = if self.is_disabled {
            &assets.button_disabled
        } else if self.is_pressed {
            &assets.button_pressed
        } else if self.is_hovered {
            &assets.button_hovered
        } else {
            &assets.button_default
        };
        assets.button.draw_nine_slice(
            ctx,
            config,
            (bounds.width + 30.0) / 3.0,
            42.0 / 3.0,
            DrawParams::new()
                .position(vec)
                .scale(TetraVec2::new(3.0, 3.0)),
        );
        vec.x += rect.w / 2.0 - bounds.width / 2.0;
        if !self.keys.is_empty() {
            vec.x -= 3.0;
        }
        vec.y += rect.h / 2.0 - bounds.height / 2.0 - 3.0;
        if !self.is_pressed {
            vec.y -= 2.0;
        }
        self.text.draw(
            ctx,
            DrawParams::new().position(vec).color(Colors::LIGHT_YELLOW),
        );
        self.dirty = false;
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Button {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> TetraVec2 {
        let text_width = self.text.get_bounds(ctx).map(|r| r.width).unwrap();
        TetraVec2::new(text_width + 30.0, 42.0)
    }
}

fn is_pressed_key_with_mod(ctx: &mut Context, key: Key, key_mod: Option<KeyModifier>) -> bool {
    if !input::is_key_pressed(ctx, key) {
        return false;
    }
    if let Some(key_mod) = key_mod {
        input::is_key_modifier_down(ctx, key_mod)
    } else {
        !input::is_key_modifier_down(ctx, KeyModifier::Alt)
            && !input::is_key_modifier_down(ctx, KeyModifier::Ctrl)
            && !input::is_key_modifier_down(ctx, KeyModifier::Shift)
    }
}

impl Update for Button {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }
    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        if self.is_disabled {
            return None;
        }
        if !self.keys.is_empty() {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for (key, key_mod) in self.keys.iter() {
                if is_pressed_key_with_mod(ctx, *key, *key_mod) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, *key) && self.is_pressed {
                    off_pressed = true
                }
            }
            if on_pressed {
                self.on_pressed();
            } else if off_pressed {
                self.off_pressed();
                return self.id();
            }
        }
        let mouse = input::get_mouse_position(ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        if collides && !self.is_pressed && input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            self.on_pressed();
        } else if self.is_pressed && input::is_mouse_button_released(ctx, MouseButton::Left) {
            self.off_pressed();
            if collides {
                return self.id();
            }
        }
        None
    }
}

impl Sprite for Button {
    fn on_pressed(&mut self) {
        self.is_pressed = true;
        self.dirty = true;
    }

    fn off_pressed(&mut self) {
        if !self.fixable {
            self.unpress();
        }
    }

    fn unpress(&mut self) {
        self.is_pressed = false;
        self.dirty = true;
    }

    fn on_hovered(&mut self) {
        self.is_hovered = true;
        self.dirty = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
        self.dirty = true;
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled != self.is_disabled {
            self.is_disabled = disabled;
            self.dirty = true;
        }
    }
}
