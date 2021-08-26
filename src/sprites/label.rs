use sprites::position::Position;
use sprites::sprite::Sprite;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::{Context, TetraVec2};

pub struct Label {
    pub text: Text,
    pub color: Color,
    pub position: Position,
    vec: Option<TetraVec2>,
}

impl Label {
    pub fn new(text: &str, font: Font, color: Color, position: Position) -> Label {
        Label {
            text: Text::new(text, font),
            color,
            position,
            vec: None,
        }
    }
}

impl Sprite for Label {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn size(&mut self, ctx: &mut Context) -> TetraVec2 {
        let rect = self.text.get_bounds(ctx).unwrap();
        TetraVec2::new(rect.width, rect.height)
    }

    fn set_vec(&mut self, vec: TetraVec2) {
        self.vec = Some(vec);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(self.vec.unwrap())
                .color(self.color),
        );
    }
}
