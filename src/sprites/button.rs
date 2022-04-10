#![allow(dead_code)]
use crate::colors::Colors;
use crate::input;
use crate::scenes::transition::Transition;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Disable, Draw, Hover, Positionate, Press, Sprite, Update};
use assets;
use assets::prepared_font::PreparedFont;
use geometry::{Rect, Vec2};
use tetra::graphics::text::Text;
use tetra::graphics::{DrawParams, Rectangle, Texture};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::Context;

enum ButtonContent {
    Text(Text, f32),
    Icon {
        region: Rectangle,
        scale: Vec2,
        tileset: Texture,
    },
    Empty(Vec2),
}

impl ButtonContent {
    pub const fn offset_x(&self) -> f32 {
        match self {
            ButtonContent::Text(..) => 30.0,
            ButtonContent::Empty(..) => 0.0,
            ButtonContent::Icon { .. } => 10.0,
        }
    }
}

enum ButtonState {
    Default,
    Pressed,
    Hovered,
    Disabled,
}

pub struct Button {
    keys: Vec<(Key, Option<KeyModifier>)>,
    content: ButtonContent,
    on_click: Transition,
    position: Position,
    asset: assets::button::Button,
    scale: Vec2,
    rect: Option<Rect>,
    is_pressed: bool,
    is_disabled: bool,
    is_hovered: bool,
    fixable: bool,
    visible: bool,
}

impl Button {
    fn new(
        keys: Vec<(Key, Option<KeyModifier>)>, // TODO: create type or struct
        content: ButtonContent,
        asset: assets::button::Button,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            keys,
            content,
            on_click,
            position,
            asset,
            scale: Vec2::new(3.0, 3.0),
            rect: None,
            is_pressed: false,
            is_hovered: false,
            is_disabled: false,
            fixable: false,
            visible: true,
        }
    }

    pub fn text(
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        font: PreparedFont,
        asset: assets::button::Button,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::Text(Text::new(text, font.font), font.line_height),
            asset,
            position,
            on_click,
        )
    }

    pub fn empty(
        keys: Vec<(Key, Option<KeyModifier>)>,
        asset: assets::button::Button,
        size: Vec2,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(keys, ButtonContent::Empty(size), asset, position, on_click)
    }

    pub fn fixed(
        keys: Vec<(Key, Option<KeyModifier>)>,
        text: &str,
        font: PreparedFont,
        asset: assets::button::Button,
        state: bool,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self {
            fixable: true,
            is_pressed: state,
            ..Self::text(keys, text, font, asset, position, on_click)
        }
    }

    pub fn icon(
        keys: Vec<(Key, Option<KeyModifier>)>,
        region: Rectangle,
        tileset: Texture,
        asset: assets::button::Button,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::Icon {
                region,
                scale: Vec2::new(3.0, 3.0),
                tileset,
            },
            asset,
            position,
            on_click,
        )
    }

    pub fn with_disabled(mut self, val: bool) -> Self {
        self.is_disabled = val;
        self
    }

    pub fn custom_event(&self) -> Option<u8> {
        if let Transition::CustomEvent(s) = self.on_click {
            Some(s)
        } else {
            None
        }
    }

    fn content_size(&mut self, ctx: &mut Context) -> Vec2 {
        match &mut self.content {
            ButtonContent::Text(text, height) => text
                .get_bounds(ctx)
                .map(|b| Vec2::new(b.width, *height))
                .unwrap(),
            ButtonContent::Empty(size) => *size,
            ButtonContent::Icon { region, scale, .. } => {
                Vec2::new(region.width * scale.x, region.height * scale.y)
            }
        }
    }

    fn state(&self) -> ButtonState {
        if self.is_disabled {
            ButtonState::Disabled
        } else if self.is_pressed {
            ButtonState::Pressed
        } else if self.is_hovered {
            ButtonState::Hovered
        } else {
            ButtonState::Default
        }
    }
}

impl Draw for Button {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let mut vec = Vec2::new(rect.x, rect.y);
        let content_size = self.content_size(ctx);

        let config = match self.state() {
            ButtonState::Default => &self.asset.default,
            ButtonState::Pressed => &self.asset.pressed,
            ButtonState::Hovered => &self.asset.hovered,
            ButtonState::Disabled => &self.asset.disabled,
        };
        self.asset.texture.draw_nine_slice(
            ctx,
            config,
            rect.w / self.scale.x,
            rect.h / self.scale.y,
            DrawParams::new().position(vec).scale(self.scale),
        );

        vec += Vec2::new(rect.w, rect.h) / 2.0 - content_size / 2.0;
        if !self.is_pressed {
            vec.y -= 2.0;
        }
        match &mut self.content {
            ButtonContent::Text(text, _) => {
                text.draw(
                    ctx,
                    DrawParams::new().position(vec).color(Colors::LIGHT_YELLOW),
                );
            }
            ButtonContent::Icon {
                region,
                scale,
                tileset,
            } => {
                vec.y -= 1.0;
                tileset.draw_region(ctx, *region, DrawParams::new().position(vec).scale(*scale));
            }
            ButtonContent::Empty(_) => {}
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
        let content_size = self.content_size(ctx);
        let offset_x = self.content.offset_x();
        Vec2::new(content_size.x + offset_x, 42.0)
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Update for Button {
    fn update(&mut self, ctx: &mut Context, focused: bool, blocked: &[Rect]) -> Option<Transition> {
        if self.is_disabled {
            return None;
        }
        if !self.keys.is_empty() && !focused {
            let mut on_pressed = false;
            let mut off_pressed = false;
            for (key, key_mod) in self.keys.iter().copied() {
                if input::is_pressed_key_with_mod(ctx, key, key_mod) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, key) && self.is_pressed {
                    off_pressed = true
                }
            }
            if on_pressed {
                self.on_pressed();
            } else if off_pressed {
                self.off_pressed();
                return Some(self.on_click.clone());
            }
        }
        let mouse = input::get_mouse_position(ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        if collides && blocked.iter().any(|r| r.contains_point(mouse)) {
            return None;
        }
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
                return Some(self.on_click.clone());
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
