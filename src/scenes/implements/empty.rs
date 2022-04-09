use colors::Colors;
use scenes::easy_back;
use scenes::scene::Scene;
use scenes::transition::Transition;
use tetra::{graphics, Context, Event};

pub struct Empty {}

impl Scene for Empty {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> Vec<Transition> {
        easy_back(event, false)
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, Colors::DARK_BROWN)
    }
}
