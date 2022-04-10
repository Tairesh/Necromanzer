#![allow(dead_code)]
pub mod game_scene;
pub mod implements;
pub mod scene;
pub mod transition;

use assets::Assets;
use colors::Colors;
use scenes::transition::{SomeTransitions, Transition};
use settings::game::GameSettings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{Position, Vertical};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, MouseButton};
use tetra::Event;
use time::{Month, OffsetDateTime};

pub(crate) fn easy_back(event: Event, focused: bool) -> SomeTransitions {
    if focused {
        return None;
    }
    match event {
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        }
        | Event::KeyPressed {
            key: Key::Escape | Key::Backspace,
        } => Some(vec![Transition::Pop]),
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

pub(crate) fn title(title: &str, assets: &Assets) -> Rc<RefCell<Label>> {
    Rc::new(RefCell::new(Label::new(
        title,
        assets.fonts.header1.clone(),
        Colors::DARK_GREEN,
        Position::horizontal_center(0.0, Vertical::ByTop { y: 20.0 }),
    )))
}

pub(crate) fn back_btn(position: Position, assets: &Assets) -> Rc<RefCell<Button>> {
    Rc::new(RefCell::new(Button::text(
        vec![(Key::Escape, None)],
        "[Esc] Back",
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::Pop,
    )))
}
