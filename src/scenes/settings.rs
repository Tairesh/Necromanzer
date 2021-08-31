use assets::Assets;
use colors::Colors;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::{Settings, WindowMode};
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position, Vertical};
use sprites::sprite::{Positionate, Press, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

pub struct SettingsScene {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    radio_buttons: Vec<Rc<RefCell<Button>>>,
}

impl SettingsScene {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        ctx: &mut Context,
    ) -> Self {
        let bg = Rc::new(RefCell::new(Image::new(
            assets.borrow().bg.clone(),
            Position::center(),
        )));
        let title = Rc::new(RefCell::new(Label::new(
            "Settings",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        )));
        let fullscreen_btn = Rc::new(RefCell::new(Button::fixed(
            "window_mode:fullscreen",
            vec![(Key::F, Some(KeyModifier::Alt))],
            "[Alt+F] Fullscreen",
            settings.borrow().window_mode() == WindowMode::Fullscreen,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 100.0 },
                y: AnchorY::Center.to_position(150.0),
            },
        )));
        let window_btn = Rc::new(RefCell::new(Button::fixed(
            "window_mode:window",
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            settings.borrow().window_mode() == WindowMode::Window,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 98.0 },
                y: AnchorY::Center.to_position(150.0),
            },
        )));
        let window_size = window_btn.borrow_mut().calc_size(ctx);
        let window_mode = Rc::new(RefCell::new(Label::new(
            "Window mode:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_size.x,
                },
                y: AnchorY::Center.to_position(145.0),
            },
        )));
        let back_btn = Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets, // no need to clone as there is last use of it
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowBottom { offset: -200.0 },
            },
        )));

        SettingsScene {
            radio_buttons: vec![window_btn.clone(), fullscreen_btn.clone()],
            sprites: vec![bg, title, window_mode, window_btn, fullscreen_btn, back_btn],
        }
    }
}

impl Scene for SettingsScene {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        if btn_id.starts_with("window_mode:") {
            for sprite in self.radio_buttons.iter() {
                if !sprite.borrow().id().eq(btn_id) {
                    sprite.borrow_mut().unpress();
                }
            }
            match btn_id.strip_prefix("window_mode:").unwrap() {
                "window" => Some(Transition::ChangeWindowMode(WindowMode::Window)),
                "fullscreen" => Some(Transition::ChangeWindowMode(WindowMode::Fullscreen)),
                _ => unreachable!(),
            }
        } else {
            match btn_id {
                "back" => Some(Transition::Pop),
                _ => None,
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Some(Transition::Pop)
        } else {
            update_sprites(self, ctx)
        }
    }

    fn sprites(&mut self) -> Option<&mut Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&mut self.sprites)
    }
}
