use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::Sprite;
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
    pub size: (f32, f32),
    scale: TetraVec2,
    pub position: Position,
    default: Texture,
    disabled: Texture,
    pressed: Texture,
    vec: Option<TetraVec2>,
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
        scale: TetraVec2,
        position: Position,
    ) -> Button {
        let text = Text::new(text, assets.borrow().consolab.clone());
        let size = assets.borrow().button.size();
        Button {
            id: id.to_string(),
            key,
            text,
            size: (size.0 as f32 * scale.x, size.1 as f32 * scale.y),
            scale,
            position,
            default: assets.borrow().button.clone(),
            disabled: assets.borrow().button_disabled.clone(),
            pressed: assets.borrow().button_pressed.clone(),
            vec: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            dirty: false,
        }
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
}

impl Sprite for Button {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn size(&mut self, _ctx: &mut Context) -> (f32, f32) {
        self.size
    }

    fn set_vec(&mut self, vec: TetraVec2) {
        self.vec = Some(vec);
    }

    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        if let Some(key) = self.key {
            if input::is_key_pressed(ctx, key) {
                self.on_pressed();
            }
            if input::is_key_released(ctx, key) {
                self.off_pressed();
                return Some(self.id.clone());
            }
            let mouse = input::get_mouse_position(ctx);
            let rect = Rect::new(
                self.vec.unwrap().x,
                self.vec.unwrap().y,
                self.size.0,
                self.size.1,
            );
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

    fn draw(&mut self, ctx: &mut Context) {
        let mut vec = self.vec.unwrap();
        let texture = if self.is_disabled {
            &self.disabled
        } else if self.is_pressed {
            &self.pressed
        } else {
            &self.default
        };
        texture.draw(ctx, DrawParams::new().position(vec).scale(self.scale));
        let bounds = self.text.get_bounds(ctx).unwrap();
        vec.x += self.size.0 / 2.0 - bounds.width / 2.0;
        vec.y += self.size.1 / 2.0 - bounds.height / 2.0 - 3.0;
        if !self.is_pressed {
            vec.y -= 2.0;
        }
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(vec)
                .color(if self.is_pressed || self.is_hovered {
                    Colors::LIGHT_YELLOW
                } else {
                    Colors::LIGHT_SEPIA
                }),
        );
        self.dirty = false;
    }
}
