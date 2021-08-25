use assets::Assets;
use colors::Colors;
use scene_manager::{Scene, Transition};
use scenes::settings::SettingsScene;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{input, window, Context};
use VERSION;

pub struct MainMenu {
    assets: Rc<RefCell<Assets>>,
    version_label: Text,
}

impl MainMenu {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        let version_label = Text::new(&*VERSION, assets.borrow().consolab.clone());
        Ok(MainMenu {
            assets,
            version_label,
        })
    }
}

impl Scene for MainMenu {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_released(ctx, MouseButton::Left) {
            Ok(Transition::Push(Box::new(SettingsScene::new(Rc::clone(
                &self.assets,
            ))?)))
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
        let logo_width = self.assets.borrow().logo.width() as f32;
        let logo_pos = Vec2::new(w as f32 / 2.0 - logo_width / 2.0, 30.0);
        self.assets.borrow().logo.draw(ctx, logo_pos);
        let bounds = self.version_label.get_bounds(ctx).unwrap();
        let version_pos = Vec2::new(
            logo_pos.x + logo_width - bounds.width - 15.0,
            logo_pos.y + 15.0,
        );
        self.version_label.draw(
            ctx,
            DrawParams::new()
                .position(version_pos)
                .color(Colors::DARK_GRAY),
        );
        Ok(())
    }
}
