use assets::Assets;
use colors::Colors;
use scene_manager::{Scene, Transition};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{input, window, Context};

pub struct SettingsScene {
    assets: Rc<RefCell<Assets>>,
    title: Text,
}

impl SettingsScene {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        let title = Text::new("Settings", assets.borrow().avqest.clone());
        Ok(SettingsScene { assets, title })
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

    fn draw(&mut self, _ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn clear(&mut self, ctx: &mut Context) -> tetra::Result {
        let (w, h) = window::get_size(ctx);
        let (bg_width, bg_height) = self.assets.borrow().bg.size();
        let bg_pos = Vec2::new(
            (w / 2 - bg_width / 2) as f32,
            (h / 2 - bg_height / 2) as f32,
        );
        self.assets.borrow().bg.draw(ctx, bg_pos);
        let bounds = self.title.get_bounds(ctx).unwrap();
        let title_pos = Vec2::new(w as f32 / 2.0 - bounds.width / 2.0, 30.0);
        self.title.draw(
            ctx,
            DrawParams::new()
                .position(title_pos)
                .color(Colors::DARK_GREEN),
        );
        Ok(())
    }
}
