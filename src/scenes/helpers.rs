use tetra::{
    input::{Key, MouseButton},
    Event,
};
use time::{Month, OffsetDateTime};

use super::{SomeTransitions, Transition};
use crate::{
    assets::Assets,
    colors::Colors,
    settings::Settings,
    ui::{Button, Image, Label, Position, TextInput, Vertical},
};

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

pub(crate) fn bg(assets: &Assets) -> Box<Image> {
    let date = OffsetDateTime::now_utc().to_offset(Settings::instance().time.offset);
    Box::new(Image::repeat(
        if date.month() == Month::October && date.day() >= 19 && date.day() <= 31 {
            assets.images.halloween.clone()
        } else {
            assets.images.bg.clone()
        },
    ))
}

pub(crate) fn title(title: impl Into<String>, assets: &Assets) -> Box<Label> {
    Box::new(Label::new(
        title,
        assets.fonts.header1.clone(),
        Colors::DARK_GREEN,
        Position::horizontal_center(0.0, Vertical::ByTop { y: 20.0 }),
    ))
}

pub(crate) fn subtitle(subtitle: impl Into<String>, assets: &Assets) -> Box<Label> {
    Box::new(Label::new(
        subtitle,
        assets.fonts.header2.clone(),
        Colors::DARK_BROWN,
        Position::horizontal_center(0.0, Vertical::ByTop { y: 100.0 }),
    ))
}

pub(crate) fn label(text: impl Into<String>, assets: &Assets, position: Position) -> Box<Label> {
    Box::new(Label::new(
        text,
        assets.fonts.header2.clone(),
        Colors::DARK_BROWN,
        position,
    ))
}

pub(crate) fn error_label(
    text: impl Into<String>,
    assets: &Assets,
    position: Position,
) -> Box<Label> {
    Box::new(Label::hidden(
        text,
        assets.fonts.default.clone(),
        Colors::RED,
        position,
    ))
}

pub(crate) fn back_btn(position: Position, assets: &Assets) -> Box<Button> {
    Box::new(Button::text(
        vec![Key::Escape.into()],
        "[Esc] Back",
        assets.fonts.default.clone(),
        assets.button.clone(),
        position,
        Transition::Pop,
    ))
}

pub(crate) fn text_input(
    value: impl Into<String>,
    width: f32,
    assets: &Assets,
    position: Position,
) -> Box<TextInput> {
    Box::new(TextInput::new(
        value,
        width,
        assets.fonts.header2.clone(),
        position,
    ))
}
