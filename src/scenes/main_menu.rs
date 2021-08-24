use assets::{Assets, FontId, TextureId};
use colors::Colors;
use scene_manager::{Scene, Transition};
use scenes::settings::SettingsScene;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::text::Text;
use tetra::graphics::DrawParams;
use tetra::input::MouseButton;
use tetra::math::Vec2;
use tetra::{graphics, input, window, Context};
use VERSION;

pub struct MainMenu {
    assets: Rc<RefCell<Assets>>,
}

impl MainMenu {
    pub fn new(assets: Rc<RefCell<Assets>>) -> tetra::Result<Self> {
        Ok(MainMenu { assets })
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

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Colors::DARK_GREEN);
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
        let (logo_width, _logo_height) = self
            .assets
            .borrow()
            .get_texture(TextureId::Logo)
            .get_data(ctx)
            .size();
        let logo_pos = DrawParams::new().position(Vec2::new((w / 2 - logo_width / 2) as f32, 30.0));
        self.assets
            .borrow()
            .get_texture(TextureId::Logo)
            .draw(ctx, logo_pos.clone());
        let mut version_text = Text::new(
            &*VERSION,
            self.assets.borrow().get_font(FontId::Default).clone(),
        );
        let bounds = version_text.get_bounds(ctx).unwrap();
        let version_pos = DrawParams::new()
            .position(Vec2::new(
                logo_pos.position.x + logo_width as f32 - bounds.width - 15.0,
                logo_pos.position.y + 15.0,
            ))
            .color(Colors::BLACK);
        version_text.draw(ctx, version_pos);
        Ok(())
    }
}
