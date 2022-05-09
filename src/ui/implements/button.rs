#![allow(dead_code)]

use std::rc::Rc;

use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::Context;

use assets::button::Button as ButtonAsset;
use assets::prepared_font::PreparedFont;
use assets::tileset::Tileset;
use geometry::{Rect, Vec2};
use input::KeyWithMod;

use crate::colors::Colors;
use crate::input;
use crate::scenes::transition::Transition;
use crate::ui::position::Position;
use crate::ui::traits::{Disable, Draw, Hover, Positionate, Press, UiSprite, Update};

enum ButtonContent {
    Text(Text, f32),
    Icon {
        name: &'static str,
        scale: Vec2,
        tileset: Rc<Tileset>,
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

#[allow(clippy::struct_excessive_bools)]
pub struct Button {
    keys: Vec<KeyWithMod>,
    content: ButtonContent,
    on_click: Transition,
    position: Position,
    asset: Rc<ButtonAsset>,
    scale: Vec2,
    rect: Option<Rect>,
    is_pressed: bool, // TODO: use state-machine
    is_disabled: bool,
    is_hovered: bool,
    fixable: bool,
    visible: bool,
}

impl Button {
    fn new(
        keys: Vec<KeyWithMod>,
        content: ButtonContent,
        asset: Rc<ButtonAsset>,
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

    pub fn text<S>(
        keys: Vec<KeyWithMod>,
        text: S,
        font: PreparedFont,
        asset: Rc<ButtonAsset>,
        position: Position,
        on_click: Transition,
    ) -> Self
    where
        S: Into<String>,
    {
        Self::new(
            keys,
            ButtonContent::Text(Text::new(text, font.font), font.line_height),
            asset,
            position,
            on_click,
        )
    }

    pub fn empty(
        keys: Vec<KeyWithMod>,
        asset: Rc<ButtonAsset>,
        size: Vec2,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(keys, ButtonContent::Empty(size), asset, position, on_click)
    }

    pub fn fixed(
        keys: Vec<KeyWithMod>,
        text: &str,
        font: PreparedFont,
        asset: Rc<ButtonAsset>,
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
        keys: Vec<KeyWithMod>,
        name: &'static str,
        tileset: Rc<Tileset>,
        asset: Rc<ButtonAsset>,
        position: Position,
        on_click: Transition,
    ) -> Self {
        Self::new(
            keys,
            ButtonContent::Icon {
                name,
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
            ButtonContent::Icon { scale, tileset, .. } => Vec2::new(
                tileset.tile_size as f32 * scale.x,
                tileset.tile_size as f32 * scale.y,
            ),
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
                name,
                scale,
                tileset,
            } => {
                vec.y -= 1.0;
                tileset.draw_region(ctx, name, DrawParams::new().position(vec).scale(*scale));
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
            for kwm in self.keys.iter().copied() {
                if input::is_key_with_mod_pressed(ctx, kwm) {
                    on_pressed = true;
                }
                if input::is_key_released(ctx, kwm.key) && self.is_pressed {
                    off_pressed = true;
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

impl UiSprite for Button {}
