use assets::{Assets, TextureId};
use colors::Colors;
use scene_manager::{Scene, Transition};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{graphics, input, window, Context};

pub struct SettingsScene {
    assets: Rc<RefCell<Assets>>,
}

impl SettingsScene {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        Ok(SettingsScene { assets })
    }
}

impl Scene for SettingsScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            Ok(Transition::Pop)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::BLACK);
        let (w, h) = window::get_size(ctx);
        let (bg_width, bg_height) = self
            .assets
            .borrow()
            .get_texture(TextureId::Background)
            .get_data(ctx)
            .size();
        let bg_pos = DrawParams::new().position(Vec2::new(
            (w / 2 - bg_width / 2) as f32,
            (h / 2 - bg_height / 2) as f32,
        ));
        self.assets
            .borrow()
            .get_texture(TextureId::Background)
            .draw(ctx, bg_pos);
        Ok(())
    }
}
