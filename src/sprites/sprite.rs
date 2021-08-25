use sprites::position::Position;
use tetra::{window, Context, TetraVec2};

pub trait Sprite {
    fn position(&self) -> Position;
    fn set_position(&mut self, position: Position);
    fn size(&mut self, ctx: &mut Context) -> (f32, f32);
    fn set_vec(&mut self, vec: TetraVec2);
    fn calc_position(&mut self, ctx: &mut Context) -> TetraVec2 {
        let window_size = window::get_size(ctx);
        let owner_size = self.size(ctx);
        let vec = self.position().vec(owner_size, window_size);
        self.set_vec(vec);
        vec
    }
    fn draw(&mut self, ctx: &mut Context);
}
