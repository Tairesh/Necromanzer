use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{DrawParams, Rectangle};
use tetra::input::MouseButton;
use tetra::{input, Context, TetraRect, TetraVec2};

pub struct Alert {
    assets: Rc<RefCell<Assets>>,
    scale: TetraVec2,
    width: f32,
    height: f32,
    position: Position,
    rect: Option<TetraRect>,
    visible: bool,
}

impl Alert {
    pub fn new(width: f32, height: f32, assets: Rc<RefCell<Assets>>, position: Position) -> Self {
        Alert {
            assets,
            scale: TetraVec2::new(3.0, 3.0),
            width,
            height,
            position,
            rect: None,
            visible: true,
        }
    }
}

impl Draw for Alert {
    fn draw(&mut self, ctx: &mut Context) {
        let assets = self.assets.borrow();
        let tl = &assets.alert_top_left;
        let t = &assets.alert_top;
        let tr = &assets.alert_top_right;
        let ml = &assets.alert_middle_left;
        let mr = &assets.alert_middle_right;
        let bl = &assets.alert_bottom_left;
        let b = &assets.alert_bottom;
        let br = &assets.alert_bottom_right;
        let rect = self.rect.unwrap();
        let mut pos = TetraVec2::new(rect.x, rect.y);
        tl.draw(ctx, DrawParams::new().position(pos).scale(self.scale));
        t.draw(
            ctx,
            DrawParams::new()
                .position(pos + TetraVec2::new(6.0 * self.scale.x, 0.0))
                .scale(self.scale * TetraVec2::new(self.width, 1.0)),
        );
        tr.draw(
            ctx,
            DrawParams::new()
                .position(pos + TetraVec2::new((self.width + 6.0) * self.scale.x, 0.0))
                .scale(self.scale),
        );
        pos += TetraVec2::new(0.0, 5.0 * self.scale.y);
        ml.draw(
            ctx,
            DrawParams::new()
                .position(pos)
                .scale(self.scale * TetraVec2::new(1.0, self.height)),
        );
        mr.draw(
            ctx,
            DrawParams::new()
                .position(pos + TetraVec2::new((self.width + 6.0) * self.scale.x, 0.0))
                .scale(self.scale * TetraVec2::new(1.0, self.height)),
        );
        Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(
                pos.x + 6.0 * self.scale.x,
                pos.y,
                self.width * self.scale.x,
                self.height * self.scale.y,
            ),
        )
        .unwrap()
        .draw(ctx, DrawParams::new().color(Colors::KHAKI));
        pos += TetraVec2::new(0.0, self.scale.y * self.height);
        bl.draw(ctx, DrawParams::new().position(pos).scale(self.scale));
        b.draw(
            ctx,
            DrawParams::new()
                .position(pos + TetraVec2::new(6.0 * self.scale.x, 0.0))
                .scale(self.scale * TetraVec2::new(self.width, 1.0)),
        );
        br.draw(
            ctx,
            DrawParams::new()
                .position(pos + TetraVec2::new(self.scale.x * (self.width + 6.0), 0.0))
                .scale(self.scale),
        );
    }

    fn set_rect(&mut self, rect: TetraRect) {
        self.rect = Some(rect);
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for Alert {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        TetraVec2::new(
            (12.0 + self.width) * self.scale.x,
            (10.0 + self.height) * self.scale.y,
        )
    }
}

impl Update for Alert {
    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            let mouse = input::get_mouse_position(ctx);
            if !self.rect.unwrap().contains_point(mouse) {
                return Some("back".to_string());
            }
        }
        None
    }
}
impl Sprite for Alert {}
