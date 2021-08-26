use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::{DrawParams, Texture};
use tetra::input::{Key, MouseButton};
use tetra::math::Rect;
use tetra::{input, Context, TetraVec2};

pub struct Button {
    pub id: String,
    pub key: Option<Key>,
    pub text: Text,
    pub position: Position,
    default_texture: Texture,
    // should be the same size as default_texture
    disabled_texture: Texture,
    pressed_texture: Texture,
    hovered_texture: Texture,
    size: (i32, i32),
    rect: Option<Rect<f32, f32>>,
    pub is_pressed: bool,
    pub is_disabled: bool,
    pub is_hovered: bool,
    dirty: bool,
}

impl Button {
    pub fn new(
        id: &str,
        key: Option<Key>,
        text: &str,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Button {
        Button {
            id: id.to_string(),
            key,
            text: Text::new(text, assets.borrow().consolab.clone()),
            position,
            default_texture: assets.borrow().button.clone(),
            disabled_texture: assets.borrow().button_disabled.clone(),
            pressed_texture: assets.borrow().button_pressed.clone(),
            hovered_texture: assets.borrow().button_hovered.clone(),
            size: assets.borrow().button.size(),
            rect: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            dirty: false,
        }
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }

    fn on_pressed(&mut self) {
        self.is_pressed = true;
        self.dirty = true;
    }

    fn off_pressed(&mut self) {
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

    fn texture(&self) -> &Texture {
        if self.is_disabled {
            &self.disabled_texture
        } else if self.is_pressed {
            &self.pressed_texture
        } else if self.is_hovered {
            &self.hovered_texture
        } else {
            &self.default_texture
        }
    }

    fn scale(&mut self, ctx: &mut Context) -> TetraVec2 {
        let bounds = self.text.get_bounds(ctx).unwrap();
        TetraVec2::new((bounds.width + 30.0) / self.texture().width() as f32, 3.0)
    }
}

impl Draw for Button {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = TetraVec2::new(rect.x, rect.y);
        let scale = self.scale(ctx);
        self.texture()
            .draw(ctx, DrawParams::new().position(vec).scale(scale));
        let bounds = self.text.get_bounds(ctx).unwrap();
        vec.x += rect.w / 2.0 - bounds.width / 2.0 - 3.0;
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
}

impl Positionate for Button {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> TetraVec2 {
        let scale = self.scale(ctx);
        TetraVec2::new(self.size.0 as f32 * scale.x, self.size.1 as f32 * scale.y)
    }
}

impl Update for Button {
    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        if self.is_disabled {
            return None;
        }
        if let Some(key) = self.key {
            if input::is_key_pressed(ctx, key) {
                self.on_pressed();
            }
            if input::is_key_released(ctx, key) {
                self.off_pressed();
                return Some(self.id.clone());
            }
            let mouse = input::get_mouse_position(ctx);
            let rect = self.rect.unwrap();
            let collides = rect.contains_point(mouse);
            if !self.is_hovered && collides {
                self.on_hovered();
            } else if self.is_hovered && !collides {
                self.off_hovered();
            }
            if collides && input::is_mouse_button_pressed(ctx, MouseButton::Left) {
                self.on_pressed();
            } else if input::is_mouse_button_released(ctx, MouseButton::Left) {
                self.off_pressed();
                if collides {
                    return Some(self.id.clone());
                }
            }
        }
        None
    }
}

impl Sprite for Button {}
