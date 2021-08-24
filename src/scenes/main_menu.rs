use colors::Colors;
use scene_manager::{Scene, Transition};
use scenes::settings::Settings;
use tetra::input::MouseButton;
use tetra::{graphics, input, Context};

pub struct MainMenu;

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            Ok(Transition::Push(Box::new(Settings {})))
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::DARK_GREEN);
        Ok(())
    }
}
