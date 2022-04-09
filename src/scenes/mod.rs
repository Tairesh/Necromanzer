#![allow(dead_code)]
pub mod game_scene;
pub mod implements;
pub mod scene;
pub mod transition;

use assets::Assets;
use scenes::transition::Transition;
use settings::game::GameSettings;
use sprites::image::Image;
use sprites::position::Position;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::Event;
use time::{Month, OffsetDateTime};

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

pub(crate) fn bg(assets: &Assets, settings: &GameSettings) -> Rc<RefCell<Image>> {
    let date = OffsetDateTime::now_utc().to_offset(settings.time_settings.offset);
    Rc::new(RefCell::new(Image::new(
        if date.month() == Month::October && date.day() >= 19 && date.day() <= 31 {
            assets.images.halloween.clone()
        } else {
            assets.images.bg.clone()
        },
        Position::center(),
    )))
}
