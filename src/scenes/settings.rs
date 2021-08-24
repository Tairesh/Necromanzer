use colors::Colors;
use scene_manager::{Scene, Transition};
use tetra::input::MouseButton;
use tetra::{graphics, input, Context};

pub struct Settings;

impl Scene for Settings {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::BLACK);
        Ok(())
    }
}
