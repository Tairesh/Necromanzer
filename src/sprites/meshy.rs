use sprites::position::Position;
use sprites::sprite::{Colorize, Draw, Hover, Positionate, Sprite, Update};
use tetra::graphics::mesh::Mesh;
use tetra::graphics::{Color, DrawParams};
use tetra::math::Rect;
use tetra::{input, Context, TetraRect, TetraVec2};

pub struct JustMesh {
    mesh: Mesh,
    color: Option<Color>,
    size: TetraVec2,
    position: Position,
    rect: Option<TetraRect>,
    visible: bool,
    scale: TetraVec2,
}

impl JustMesh {
    pub fn new(mesh: Mesh, color: Option<Color>, size: TetraVec2, position: Position) -> Self {
        Self {
            mesh,
            color,
            size,
            position,
            rect: None,
            visible: true,
            scale: TetraVec2::one(),
        }
    }
}

impl Draw for JustMesh {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.mesh.draw(
            ctx,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .scale(self.scale)
                .color(self.color.unwrap_or(Color::WHITE)),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for JustMesh {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        self.size * self.scale
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Colorize for JustMesh {
    fn color(&self) -> Color {
        self.color.unwrap_or(Color::WHITE)
    }

    fn set_color(&mut self, value: Color) {
        self.color = Some(value);
    }
}

impl Update for JustMesh {}
impl Sprite for JustMesh {}

pub struct HoverableMesh {
    mesh: Mesh,
    bg_color: Color,
    bg_color_hover: Color,
    size: TetraVec2,
    position: Position,
    rect: Option<TetraRect>,
    visible: bool,
    is_hovered: bool,
}

impl HoverableMesh {
    pub fn new(
        mesh: Mesh,
        bg_color: Color,
        bg_color_hover: Color,
        size: TetraVec2,
        position: Position,
    ) -> Self {
        Self {
            mesh,
            bg_color,
            bg_color_hover,
            size,
            position,
            rect: None,
            visible: true,
            is_hovered: false,
        }
    }
}

impl Draw for HoverableMesh {
    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        self.mesh.draw(
            ctx,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .color(if self.is_hovered {
                    self.bg_color_hover
                } else {
                    self.bg_color
                }),
        );
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for HoverableMesh {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        self.size
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Update for HoverableMesh {
    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        let mouse = input::get_mouse_position(ctx);
        let rect = self.rect.unwrap();
        let collides = rect.contains_point(mouse);
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        None
    }
}

impl Hover for HoverableMesh {
    fn on_hovered(&mut self) {
        self.is_hovered = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
    }
}

impl Sprite for HoverableMesh {}
