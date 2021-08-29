use sprites::position::Position;
use tetra::math::Rect;
use tetra::{Context, TetraVec2};

pub trait Draw {
    /// Need redraw
    fn dirty(&self) -> bool {
        false
    }
    fn draw(&mut self, ctx: &mut Context);
    fn set_rect(&mut self, rect: Rect<f32, f32>);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, visible: bool);
}

pub trait Positionate {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn calc_size(&mut self, ctx: &mut Context) -> TetraVec2;
    fn calc_rect(&mut self, owner_size: TetraVec2, window_size: (i32, i32)) -> Rect<f32, f32> {
        let left_top = self.position().as_vec(owner_size, window_size);
        Rect::new(left_top.x, left_top.y, owner_size.x, owner_size.y)
    }
}

pub trait Update {
    fn id(&self) -> Option<String> {
        None
    }
    fn update(&mut self, _ctx: &mut Context) -> Option<String> {
        None
    }
}

pub trait Sprite: Draw + Positionate + Update {
    fn on_pressed(&mut self) {}

    fn off_pressed(&mut self) {}

    fn unpress(&mut self) {}

    fn on_hovered(&mut self) {}

    fn off_hovered(&mut self) {}

    fn is_focusable(&self) -> bool {
        false
    }

    fn set_value(&mut self, _value: &str) {}

    fn get_value(&self) -> Option<String> {
        None
    }

    fn validate_value(&mut self) {}

    fn set_danger(&mut self, _danger: bool) {}

    fn get_danger(&self) -> bool {
        false
    }

    fn set_disabled(&mut self, _disabled: bool) {}
}
