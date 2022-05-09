#![allow(dead_code)]

use std::cell::RefCell;
use std::rc::Rc;

use tetra::input::{Key, MouseButton};
use tetra::Event;
use time::{Month, OffsetDateTime};

use assets::Assets;
use colors::Colors;
use scenes::transition::{SomeTransitions, Transition};
use settings::game::Settings;
use ui::button::Button;
use ui::image::Image;
use ui::label::Label;
use ui::position::{Position, Vertical};

mod game_modes;
pub mod implements;
pub mod scene;
pub mod scene_impl;
pub mod transition;

pub(crate) fn easy_back(event: &Event, focused: bool) -> SomeTransitions {
    if focused {
        return None;
    }
    match event {
        Event::MouseButtonPressed {
            button: MouseButton::X1,
        }
        | Event::KeyPressed {
            key: Key::Backspace,
        } => Some(vec![Transition::Pop]),
        _ => None,
    }
}

pub(crate) fn bg(assets: &Assets, settings: &Settings) -> Rc<RefCell<Image>> {
    let date = OffsetDateTime::now_utc().to_offset(settings.time.offset);
    Rc::new(RefCell::new(Image::repeat(
        if date.month() == Month::October && date.day() >= 19 && date.day() <= 31 {
            assets.images.halloween.clone()
        } else {
            assets.images.bg.clone()
        },
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
        vec![Key::Escape.into()],
        "[Esc] Back",
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::Pop,
    )))
}
