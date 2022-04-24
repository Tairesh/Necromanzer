#![allow(dead_code)]
use crate::assets::prepared_font::PreparedFont;
use crate::sprites::position::Position;
use crate::sprites::sprite::{Colorize, Draw, Positionate, Sprite, Stringify, Update};
use assets::tileset::Tileset;
use geometry::{Rect, Vec2};
use map::item::{Item, ItemView};
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle, Texture};
use tetra::Context;

pub struct Label {
    text: Text,
    color: Color,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
    line_height: f32,
}

impl Label {
    pub fn new<C: Into<String>>(
        text: C,
        font: PreparedFont,
        color: Color,
        position: Position,
    ) -> Self {
        Self {
            text: Text::new(text, font.font),
            color,
            position,
            rect: None,
            visible: true,
            line_height: font.line_height,
        }
    }

    pub fn hidden<C: Into<String>>(
        text: C,
        font: PreparedFont,
        color: Color,
        position: Position,
    ) -> Self {
        Self {
            visible: false,
            ..Self::new(text, font, color, position)
        }
    }

    pub fn update<S: Into<String>>(&mut self, text: S, ctx: &mut Context, window_size: (i32, i32)) {
        self.set_value(text);
        self.positionate(ctx, window_size);
    }
}

impl Draw for Label {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(rect.x, rect.y))
                .color(self.color),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Label {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let rect = self.text.get_bounds(ctx).unwrap();
        Vec2::new(rect.width, f32::max(self.line_height, rect.height))
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for Label {
    fn color(&self) -> Color {
        self.color
    }

    fn set_color<C: Into<Color>>(&mut self, value: C) {
        self.color = value.into();
    }
}

impl Stringify for Label {
    fn value(&self) -> String {
        self.text.content().to_string()
    }

    fn set_value<C: Into<String>>(&mut self, value: C) {
        self.text.set_content(value);
    }
}

impl Update for Label {
    fn block_mouse(&self) -> bool {
        false
    }
}
impl Sprite for Label {}

pub struct ItemDisplay {
    text: Text,
    color: Color,
    icon: bool,
    tileset: Texture,
    region: Rectangle,
    scale: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl ItemDisplay {
    pub fn new(
        item: Option<&Item>,
        font: PreparedFont,
        color: Color,
        tileset: &Tileset,
        scale: Vec2,
        position: Position,
    ) -> Self {
        let (name, region) = if let Some(item) = item {
            (item.name(), item.region(tileset))
        } else {
            ("(empty)".to_string(), Rectangle::default())
        };
        Self {
            text: Text::new(name, font.font),
            color,
            icon: item.is_some(),
            tileset: tileset.texture.clone(),
            region,
            scale,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn set_item(
        &mut self,
        item: Option<&Item>,
        ctx: &mut Context,
        tileset: &Tileset,
        window_size: (i32, i32),
    ) {
        let (name, region) = if let Some(item) = item {
            (item.name(), Some(item.region(tileset)))
        } else {
            ("(empty)".to_string(), None)
        };
        if name != self.text.content() || region.is_some() != self.icon {
            if let Some(region) = region {
                self.icon = true;
                self.region = region;
            } else {
                self.icon = false;
            }
            self.text.set_content(name);
            self.positionate(ctx, window_size);
        }
    }
}

impl Draw for ItemDisplay {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let text_pos = if self.icon {
            self.tileset.draw_region(
                ctx,
                self.region,
                DrawParams::new()
                    .position(Vec2::new(rect.x, rect.y))
                    .scale(self.scale),
            );
            Vec2::new(rect.x + self.region.width * self.scale.x + 5.0, rect.y)
        } else {
            Vec2::new(rect.x, rect.y)
        };
        self.text
            .draw(ctx, DrawParams::new().position(text_pos).color(self.color));
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for ItemDisplay {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        let rect = self.text.get_bounds(ctx).unwrap();
        if self.icon {
            Vec2::new(
                self.region.width * self.scale.x + 5.0 + rect.width,
                rect.height,
            )
        } else {
            Vec2::new(rect.width, rect.height)
        }
    }

    fn rect(&self) -> Rect {
        self.rect.unwrap()
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for ItemDisplay {
    fn color(&self) -> Color {
        self.color
    }

    fn set_color<C: Into<Color>>(&mut self, color: C) {
        self.color = color.into();
    }
}

impl Update for ItemDisplay {}
impl Sprite for ItemDisplay {}
