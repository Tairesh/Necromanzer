use assets::Assets;
use sprites::position::{AnchorX, AnchorY, Position};
use sprites::sprite::{Colorize, Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::{Color, DrawParams, NineSlice, Rectangle, Texture};
use tetra::Context;
use {Rect, Vec2};

pub struct Image {
    texture: Texture,
    region: Option<Rectangle>,
    color: Option<Color>,
    nine_slice: Option<(NineSlice, f32, f32)>,
    scale: Vec2,
    position: Position,
    rect: Option<Rect>,
    visible: bool,
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Self {
        Image {
            texture,
            region: None,
            color: None,
            nine_slice: None,
            scale: Vec2::new(1.0, 1.0),
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_nineslice(mut self, nineslice: NineSlice, width: f32, height: f32) -> Self {
        self.nine_slice = Some((nineslice, width, height));
        self
    }
}

impl Draw for Image {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        let params = DrawParams::new()
            .position(Vec2::new(rect.x, rect.y))
            .scale(self.scale)
            .color(self.color.unwrap_or(Color::WHITE));
        if let Some((nine_slice, width, height)) = &self.nine_slice {
            self.texture
                .draw_nine_slice(ctx, nine_slice, *width, *height, params);
        } else if let Some(region) = self.region {
            self.texture.draw_region(ctx, region, params);
        } else {
            self.texture.draw(ctx, params);
        }
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Image {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> Vec2 {
        let size = if let Some(region) = self.region {
            (region.width, region.height)
        } else {
            let (w, h) = self.texture.size();
            (w as f32, h as f32)
        };
        Vec2::new(size.0 * self.scale.x, size.1 * self.scale.y)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}

impl Colorize for Image {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }
}

impl Update for Image {}
impl Sprite for Image {}

pub struct Bar {
    image: Image,
    min_width: f32,
    max_width: f32,
    max_value: u32,
    value: u32,
}

impl Bar {
    pub fn red(max_value: u32, value: u32, assets: Rc<RefCell<Assets>>) -> Self {
        let min_width = 4.0;
        let max_width = 50.0;
        Self {
            image: Image::new(
                assets.borrow().bars.clone(),
                Position::new(100.0, 8.0, AnchorX::Left, AnchorY::Top),
            )
            .with_scale(Vec2::new(4.0, 4.0))
            .with_nineslice(
                assets.borrow().bar_red.clone(),
                (value as f32 / max_value as f32) * (max_width - min_width) + min_width,
                3.0,
            ),
            min_width,
            max_width,
            max_value,
            value,
        }
    }

    pub fn blue(max_value: u32, value: u32, assets: Rc<RefCell<Assets>>) -> Self {
        let min_width = 4.0;
        let max_width = 43.0;
        Self {
            image: Image::new(
                assets.borrow().bars.clone(),
                Position::new(100.0, 32.0, AnchorX::Left, AnchorY::Top),
            )
            .with_scale(Vec2::new(4.0, 4.0))
            .with_nineslice(
                assets.borrow().bar_blue.clone(),
                (value as f32 / max_value as f32) * (max_width - min_width) + min_width,
                3.0,
            ),
            min_width,
            max_width,
            max_value,
            value,
        }
    }

    #[allow(dead_code)]
    pub fn set_value(&mut self, value: u32) {
        if value > 0 {
            self.image.set_visible(true);
            self.image.nine_slice.as_mut().unwrap().1 = (value as f32 / self.max_value as f32)
                * (self.max_width - self.min_width)
                + self.min_width;
        } else {
            self.image.set_visible(false);
        }
        self.value = value;
    }

    #[allow(dead_code)]
    pub fn value(&self) -> u32 {
        self.value
    }
}

impl Draw for Bar {
    fn draw(&mut self, ctx: &mut Context) {
        self.image.draw(ctx);
    }

    fn visible(&self) -> bool {
        self.image.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.image.visible = visible;
    }
}

impl Positionate for Bar {
    fn position(&self) -> Position {
        self.image.position
    }

    fn set_position(&mut self, position: Position) {
        self.image.position = position;
    }

    fn calc_size(&mut self, ctx: &mut Context) -> Vec2 {
        self.image.calc_size(ctx)
    }

    fn set_rect(&mut self, rect: Rect) {
        self.image.set_rect(rect);
    }
}

impl Update for Bar {}
impl Sprite for Bar {}
