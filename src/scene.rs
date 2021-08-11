use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use textures::TextureManager;
use VERSION;

pub enum CallResult {
    DoNothing,
    ChangeScene(Scene),
}

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, event: &Event) -> CallResult;
    fn update(
        &mut self,
        canvas: &mut WindowCanvas,
        textures: &mut TextureManager,
        elapsed_dime: f64,
    ) -> CallResult;
}

#[derive(Hash, Eq, PartialEq)]
pub struct MainMenu;
impl SceneT for MainMenu {
    fn call(&mut self, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => CallResult::ChangeScene(EmptyScreen {}.into()),
            _ => CallResult::DoNothing,
        }
    }

    fn update(
        &mut self,
        canvas: &mut WindowCanvas,
        textures: &mut TextureManager,
        _elapsed_dime: f64,
    ) -> CallResult {
        let (w, h) = canvas.output_size().unwrap();
        let screen_center = (w as i32 / 2, h as i32 / 2);

        let bg = textures.load_image("res/img/bg.jpg");
        let bg_position = Rect::new(
            screen_center.0 - bg.query().width as i32 / 2,
            screen_center.1 - bg.query().height as i32 / 2,
            bg.query().width,
            bg.query().height,
        );
        canvas.copy(bg, None, Some(bg_position)).ok();

        let logo = textures.load_image("res/img/logo.png");
        let logo_position = Rect::new(
            screen_center.0 - logo.query().width as i32 / 2,
            10,
            logo.query().width,
            logo.query().height,
        );
        canvas.copy(logo, None, Some(logo_position)).ok();

        let version_label = textures.render_text(VERSION, None);
        let version_position = Rect::new(
            logo_position.right() - version_label.query().width as i32 - 15,
            logo_position.top() + 25,
            version_label.query().width,
            version_label.query().height,
        );
        canvas
            .copy(version_label, None, Some(version_position))
            .ok();

        CallResult::DoNothing
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct EmptyScreen;
impl SceneT for EmptyScreen {
    fn call(&mut self, event: &Event) -> CallResult {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => CallResult::ChangeScene(MainMenu {}.into()),
            _ => CallResult::DoNothing,
        }
    }

    fn update(
        &mut self,
        canvas: &mut WindowCanvas,
        _textures: &mut TextureManager,
        _elapsed_dime: f64,
    ) -> CallResult {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        CallResult::DoNothing
    }
}

#[enum_dispatch::enum_dispatch(SceneT)]
#[derive(Hash, Eq, PartialEq)]
pub enum Scene {
    MainMenu,
    EmptyScreen,
}
