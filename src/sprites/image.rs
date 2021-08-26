use sprites::position::Position;
use sprites::sprite::Sprite;
use tetra::graphics::{DrawParams, Texture};
use tetra::{Context, TetraVec2};

pub struct Image {
    pub texture: Texture,
    size: TetraVec2,
    scale: TetraVec2,
    pub position: Position,
    vec: Option<TetraVec2>,
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Image {
        let size = texture.size();
        Image {
            texture,
            size: TetraVec2::new(size.0 as f32, size.1 as f32),
            scale: TetraVec2::new(1.0, 1.0),
            position,
            vec: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_scale(mut self, scale: TetraVec2) -> Image {
        self.size *= scale;
        self
    }
}

impl Sprite for Image {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        self.size
    }

    fn set_vec(&mut self, vec: TetraVec2) {
        self.vec = Some(vec);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(self.vec.unwrap())
                .scale(self.scale),
        );
    }
}
