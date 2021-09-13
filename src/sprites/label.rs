use assets::TilesetRegions;
use map::item::Item;
use sprites::position::Position;
use sprites::sprite::{Colorize, Draw, Positionate, Sprite, Stringify, Update};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams, Rectangle, Texture};
use tetra::{window, Context};
use {Rect, Vec2};

pub struct Label {
    text: Text,
    color: Color,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Label {
    pub fn new<C: Into<String>>(text: C, font: Font, color: Color, position: Position) -> Self {
        Label {
            text: Text::new(text, font),
            color,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn hidden<C: Into<String>>(text: C, font: Font, color: Color, position: Position) -> Self {
        Label {
            text: Text::new(text, font),
            color,
            position,
            rect: None,
            visible: false,
        }
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
        Vec2::new(rect.width, rect.height)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for Label {
    fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
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

impl Update for Label {}
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
        font: Font,
        color: Color,
        tileset: Texture,
        regions: &TilesetRegions,
        scale: Vec2,
        position: Position,
    ) -> Self {
        let (name, region) = if let Some(item) = item {
            (item.item_type.name(), item.item_type.region(regions))
        } else {
            ("(empty)".to_string(), Rectangle::default())
        };
        Self {
            text: Text::new(name, font),
            color,
            icon: item.is_some(),
            tileset,
            region,
            scale,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn set_item(&mut self, item: Option<&Item>, ctx: &mut Context, regions: &TilesetRegions) {
        let (name, region) = if let Some(item) = item {
            (item.item_type.name(), Some(item.item_type.region(regions)))
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
            self.positionate(ctx, window::get_size(ctx));
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

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for ItemDisplay {
    fn color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

impl Update for ItemDisplay {}
impl Sprite for ItemDisplay {}
