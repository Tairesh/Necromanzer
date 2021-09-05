use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Disable, Draw, Hover, Positionate, Press, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};
use {Rect, Vec2};

enum ButtonContent {
    Text(Text),
    Icon { region: Rectangle, scale: Vec2 },
}

pub struct Button {
    id: String,
    keys: Vec<(Key, Option<KeyModifier>)>,
    content: ButtonContent,
    content_height: f32,
    position: Position,
    assets: Rc<RefCell<Assets>>,
    rect: Option<Rect>,
    is_pressed: bool,
    is_disabled: bool,
    is_hovered: bool,
    fixable: bool,
    visible: bool,
}

impl Button {
    fn create(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        content: ButtonContent,
        content_height: f32,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        Self {
            id: id.to_string(),
            keys,
            content,
            content_height,
            position,
            assets,
            rect: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            fixable: false,
            visible: true,
        }
    }

    pub fn new(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        let content = ButtonContent::Text(Text::new(text, assets.borrow().default.clone()));
        Self::create(id, keys, content, 20.0, assets, position)
    }

    pub fn fixed(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        state: bool,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        let mut s = Self::new(id, keys, text, assets, position);
        s.fixable = true;
        s.is_pressed = state;
        s
    }

    pub fn icon(
        id: &str,
        keys: Vec<(Key, Option<KeyModifier>)>,
        region: Rectangle,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        let icon_scale = Vec2::new(3.0, 3.0);
        let content = ButtonContent::Icon {
            region,
            scale: icon_scale,
        };
        Self::create(
            id,
            keys,
            content,
            region.height * icon_scale.y,
            assets,
            position,
        )
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    fn content_size(&mut self, ctx: &mut Context) -> (Vec2, f32) {
        match &mut self.content {
            ButtonContent::Text(text) => (
                text.get_bounds(ctx)
                    .map(|b| Vec2::new(b.width, self.content_height))
                    .unwrap(),
                30.0f32,
            ),
            ButtonContent::Icon { region, scale } => (
                Vec2::new(region.width * scale.x, region.height * scale.y),
                10.0f32,
            ),
        }
    }
}

impl Draw for Button {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = Vec2::new(rect.x, rect.y);
        let (content_size, offset_x) = self.content_size(ctx);
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
            (content_size.x + offset_x) / 3.0,
            42.0 / 3.0,
            DrawParams::new().position(vec).scale(Vec2::new(3.0, 3.0)),
        );
        vec.x += rect.w / 2.0 - content_size.x / 2.0;
        vec.y += rect.h / 2.0 - content_size.y / 2.0;
        if !self.is_pressed {
            vec.y -= 2.0;
        }
        match &mut self.content {
            ButtonContent::Text(text) => {
                if !self.keys.is_empty() {
                    vec.x -= 3.0;
                }
                text.draw(
                    ctx,
                    DrawParams::new().position(vec).color(Colors::LIGHT_YELLOW),
                );
            }
            ButtonContent::Icon { region, scale } => {
                assets.tileset.draw_region(
                    ctx,
                    *region,
                    DrawParams::new().position(vec).scale(*scale),
                );
            }
        }
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

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let (content_size, offset_x) = self.content_size(ctx);
        Vec2::new(content_size.x + offset_x, 42.0)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
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
                return Some(self.id());
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
                return Some(self.id());
            }
        }
        None
    }
}

impl Disable for Button {
    fn disabled(&self) -> bool {
        self.is_disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        if disabled != self.is_disabled {
            self.is_disabled = disabled;
        }
    }
}

impl Hover for Button {
    fn on_hovered(&mut self) {
        self.is_hovered = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
    }
}

impl Press for Button {
    fn on_pressed(&mut self) {
        self.is_pressed = true;
    }

    fn off_pressed(&mut self) {
        if !self.fixable {
            self.unpress();
        }
    }

    fn unpress(&mut self) {
        self.is_pressed = false;
    }
}

impl Sprite for Button {}
