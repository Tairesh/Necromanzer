use assets::Assets;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::DrawParams;
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
        let rect = self.rect.unwrap();
        assets.alert.draw_nine_slice(
            ctx,
            &assets.alert_nineslice,
            self.width / self.scale.x,
            self.height / self.scale.y,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .scale(self.scale),
        )
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
        TetraVec2::new(self.width, self.height)
    }

    fn set_rect(&mut self, rect: TetraRect) {
        self.rect = Some(rect);
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
