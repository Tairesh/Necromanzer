use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::math::Rect;
use tetra::{Context, TetraVec2};

pub struct Label {
    text: Text,
    color: Color,
    position: Position,
    rect: Option<Rect<f32, f32>>,
    visible: bool,
}

impl Label {
    pub fn new(text: &str, font: Font, color: Color, position: Position) -> Self {
        Label {
            text: Text::new(text, font),
            color,
            position,
            rect: None,
            visible: true,
        }
    }

    pub fn hidden(text: &str, font: Font, color: Color, position: Position) -> Self {
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
                .position(TetraVec2::new(rect.x, rect.y))
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

    fn calc_size(&mut self, ctx: &mut Context) -> TetraVec2 {
        let rect = self.text.get_bounds(ctx).unwrap();
        TetraVec2::new(rect.width, rect.height)
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Update for Label {}
impl Sprite for Label {
    fn set_value(&mut self, value: &str) {
        self.text.set_content(value);
    }

    fn get_value(&self) -> Option<String> {
        Some(self.text.content().to_string())
    }

    fn get_color(&self) -> Option<Color> {
        Some(self.color)
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
