use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use tetra::graphics::{DrawParams, Texture};
use tetra::math::Rect;
use tetra::{Context, TetraVec2};

pub struct Image {
    texture: Texture,
    scale: TetraVec2,
    position: Position,
    rect: Option<Rect<f32, f32>>,
    visible: bool,
}

impl Image {
    pub fn new(texture: Texture, position: Position) -> Self {
        Image {
            texture,
            scale: TetraVec2::new(1.0, 1.0),
            position,
            rect: None,
            visible: true,
        }
    }

    #[allow(dead_code)]
    pub fn with_scale(mut self, scale: TetraVec2) -> Image {
        self.scale = scale;
        self
    }
}

impl Draw for Image {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.texture.draw(
            ctx,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .scale(self.scale),
        );
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

    fn calc_size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        let size = self.texture.size();
        TetraVec2::new(size.0 as f32 * self.scale.x, size.1 as f32 * self.scale.y)
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Update for Image {}
impl Sprite for Image {}
