#![allow(dead_code)]
pub mod game_scene;
pub mod implements;
pub mod scene;
pub mod transition;

use assets::Assets;
use scenes::transition::Transition;
use sprites::image::Image;
use sprites::position::Position;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::Event;

pub(crate) fn easy_back(event: Event, focused: bool) -> Option<Transition> {
    if focused {
        return None;
    }
    match event {
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        }
        | Event::KeyPressed {
            key: Key::Escape | Key::Backspace,
        } => Some(Transition::Pop),
        _ => None,
    }
}

pub(crate) fn bg(assets: &Assets) -> Rc<RefCell<Image>> {
    Rc::new(RefCell::new(Image::new(
        assets.images.bg.clone(),
        Position::center(),
    )))
}
