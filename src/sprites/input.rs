use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::math::Rect;
use tetra::{input, Context, TetraVec2};

pub struct TextInput {
    id: String,
    value: String,
    text: Text,
    position: Position,
    width: f32,
    rect: Option<Rect<f32, f32>>,
    is_focused: bool,
    is_disabled: bool,
    is_hovered: bool,
    blink: bool,
    last_blinked: Instant,
    dirty: bool,
}

impl TextInput {
    pub fn new(
        id: &str,
        value: &str,
        width: f32,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        Self {
            id: id.to_ascii_lowercase(),
            value: value.to_string(),
            text: Text::new(value, assets.borrow().default.clone()),
            position,
            width,
            rect: None,
            is_focused: false,
            is_disabled: false,
            is_hovered: false,
            blink: false,
            last_blinked: Instant::now(),
            dirty: false,
        }
    }

    fn border_color(&self) -> Color {
        if self.is_disabled {
            Colors::DARK_GRAY
        } else if self.is_focused {
            Colors::DARK_GREEN
        } else {
            Colors::DARK_BROWN
        }
    }

    fn bg_color(&self) -> Option<Color> {
        if self.is_disabled {
            Some(Colors::DARK_GRAY.with_alpha(0.8))
        } else if self.is_focused {
            Some(Colors::DARK_GREEN.with_alpha(0.8))
        } else if self.is_hovered {
            Some(Colors::DARK_BROWN.with_alpha(0.2))
        } else {
            None
        }
    }

    fn text_color(&self) -> Color {
        if self.is_disabled {
            Colors::WHITE
        } else if self.is_focused {
            Colors::LIGHT_YELLOW
        } else {
            Colors::DARK_BROWN
        }
    }
}

impl Draw for TextInput {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        if let Some(bg_color) = self.bg_color() {
            let bg = Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, rect.w, rect.h),
                BorderRadii::new(5.0),
            )
            .unwrap();
            bg.draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(rect.x, rect.y))
                    .color(bg_color),
            );
        }

        let border = Mesh::rounded_rectangle(
            ctx,
            ShapeStyle::Stroke(2.0),
            Rectangle::new(0.0, 0.0, rect.w, rect.h),
            BorderRadii::new(5.0),
        )
        .unwrap();
        border.draw(
            ctx,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .color(self.border_color()),
        );
        self.text.set_content(self.value.replace(" ", "_"));
        let text_width = self
            .text
            .get_bounds(ctx)
            .map(|r| r.width + 3.0)
            .unwrap_or(-1.0f32);
        let text_pos = if !self.is_focused || self.is_disabled {
            TetraVec2::new(rect.x + rect.w / 2.0 - text_width / 2.0, rect.y + 6.0)
        } else {
            TetraVec2::new(rect.x + 7.0, rect.y + 6.0)
        };
        self.text.set_content(self.value.as_str());
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(text_pos)
                .color(self.text_color()),
        );
        if self.blink {
            Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(text_width + 7.0, 5.0, 10.0, 20.0),
            )
            .unwrap()
            .draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(rect.x, rect.y))
                    .color(self.text_color()),
            );
        }
        self.dirty = false;
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Positionate for TextInput {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> TetraVec2 {
        self.text.set_content("Test");
        let text_height = self.text.get_bounds(ctx).map(|r| r.height).unwrap();
        self.text.set_content(self.value.as_str());
        TetraVec2::new(self.width, text_height + 20.0)
    }
}

impl Update for TextInput {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        let mouse = input::get_mouse_position(ctx);
        let collides = self.rect.unwrap().contains_point(mouse);
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        if self.is_focused {
            if input::is_mouse_button_pressed(ctx, MouseButton::Left) && !collides {
                self.off_pressed();
            }
            if Instant::now() - self.last_blinked > Duration::new(0, 500_000_000) {
                self.blink = !self.blink;
                self.last_blinked = Instant::now();
                self.dirty = true;
            }
            if input::is_key_pressed(ctx, Key::Backspace) && !self.value.is_empty() {
                let mut chars = self.value.chars();
                chars.next_back();
                self.value = chars.as_str().to_string();
                self.text.set_content(self.value.as_str());
                self.dirty = true;
            }
            if let Some(text_input) = input::get_text_input(ctx) {
                self.value.push_str(text_input);
                self.text.set_content(self.value.as_str());
                self.dirty = true;
            }
            if (input::is_key_pressed(ctx, Key::V)
                && input::is_key_modifier_down(ctx, KeyModifier::Ctrl))
                || (input::is_key_pressed(ctx, Key::Insert)
                    && input::is_key_modifier_down(ctx, KeyModifier::Shift))
            {
                self.value
                    .extend(input::get_clipboard_text(ctx).unwrap().chars().map(|c| {
                        if c == '\n' {
                            ' '
                        } else {
                            c
                        }
                    }));
                self.text.set_content(self.value.as_str());
                self.dirty = true;
            }
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Left) && collides {
            self.on_pressed();
        }
        None
    }
}

impl Sprite for TextInput {
    fn on_pressed(&mut self) {
        self.is_focused = true;
        self.blink = true;
        self.last_blinked = Instant::now();
        self.dirty = true;
    }

    fn off_pressed(&mut self) {
        self.is_focused = false;
        self.blink = false;
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

    fn is_focusable(&self) -> bool {
        !self.is_disabled
    }

    fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
        self.text.set_content(value);
        self.dirty = true;
    }

    fn get_value(&self) -> Option<String> {
        Some(self.value.clone())
    }
}
