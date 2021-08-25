use sprites::position::Position;
use sprites::sprite::Sprite;
use tetra::graphics::Texture;
use tetra::math::Vec2;
use tetra::Context;

pub struct Image {
    pub texture: Texture,
    size: (f32, f32),
    pub position: Position,
    vec: Option<Vec2<f32>>,
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Image {
        let size = texture.size();
        Image {
            texture,
            size: (size.0 as f32, size.1 as f32),
            position,
            vec: None,
        }
    }
}

impl Sprite for Image {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn size(&mut self, _ctx: &mut Context) -> (f32, f32) {
        self.size
    }

    fn set_vec(&mut self, vec: Vec2<f32>) {
        self.vec = Some(vec);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(ctx, self.vec.unwrap());
    }
}
