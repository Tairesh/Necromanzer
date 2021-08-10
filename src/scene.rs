use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub enum CallResult {
    DoNothing,
    ChangeScene(Scene),
}

#[enum_dispatch::enum_dispatch]
pub trait SceneT {
    fn call(&mut self, event: &Event) -> CallResult;
    fn update(&mut self, ecanvas: &mut WindowCanvas, elapsed_dime: f64) -> CallResult;
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

    fn update(&mut self, canvas: &mut WindowCanvas, _elapsed_dime: f64) -> CallResult {
        canvas.set_draw_color(Color::RGB(123, 255, 0));
        canvas.clear();
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

    fn update(&mut self, canvas: &mut WindowCanvas, _elapsed_dime: f64) -> CallResult {
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
