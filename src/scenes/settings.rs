use assets::Assets;
use colors::Colors;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::{Settings, WindowMode};
use sprites::button::Button;
use sprites::image::Image;
use sprites::label::Label;
use sprites::position::{AnchorY, Horizontal, Position, Vertical};
use sprites::sprite::{Positionate, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, Context};

pub struct SettingsScene {
    sprites: Vec<Box<dyn Sprite>>,
}

impl SettingsScene {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        ctx: &mut Context,
    ) -> Self {
        let bg = Image::new(assets.borrow().bg.clone(), Position::center());
        let title = Label::new(
            "Settings",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        );
        let mut fullscreen_btn = Button::fixed(
            "window_mode:fullscreen",
            vec![(Key::F, Some(KeyModifier::Alt))],
            "[Alt+F] Fullscreen",
            settings.borrow().window_mode() == WindowMode::Fullscreen,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(150.0),
            },
        );
        let fullscreen_size = fullscreen_btn.calc_size(ctx);
        let mut window_btn = Button::fixed(
            "window_mode:window",
            vec![(Key::W, Some(KeyModifier::Alt))],
            "[Alt+W] Window",
            settings.borrow().window_mode() == WindowMode::Window,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 0.0 },
                y: AnchorY::Center.to_position(150.0),
            },
        );
        let window_size = window_btn.calc_size(ctx);
        let window_mode = Label::new(
            "Window mode:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -window_size.x - 10.0,
                },
                y: AnchorY::Center.to_position(145.0),
            },
        );
        let borderless_btn = Button::fixed(
            "window_mode:borderless",
            vec![(Key::B, Some(KeyModifier::Alt))],
            "[Alt+B] Borderless",
            settings.borrow().window_mode() == WindowMode::Borderless,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: fullscreen_size.x,
                },
                y: AnchorY::Center.to_position(150.0),
            },
        );

        let back_btn = Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets, // no need to clone as there is last use of it
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: Vertical::AtWindowBottom { offset: -200.0 },
            },
        );

        SettingsScene {
            sprites: vec![
                Box::new(bg),
                Box::new(title),
                Box::new(window_mode),
                Box::new(window_btn),
                Box::new(fullscreen_btn),
                Box::new(borderless_btn),
                Box::new(back_btn),
            ],
        }
    }
}

impl Scene for SettingsScene {
    fn on_button_click(&mut self, _ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        if btn_id.starts_with("window_mode:") {
            for sprite in self.sprites[3..=5].iter_mut() {
                if let Some(other_id) = sprite.id() {
                    if other_id.as_str() != btn_id {
                        sprite.unpress();
                    }
                }
            }
            match btn_id.strip_prefix("window_mode:").unwrap() {
                "window" => Some(Transition::ChangeWindowMode(WindowMode::Window)),
                "fullscreen" => Some(Transition::ChangeWindowMode(WindowMode::Fullscreen)),
                "borderless" => Some(Transition::ChangeWindowMode(WindowMode::Borderless)),
                _ => panic!("Unimplemented window mode: {}", btn_id),
            }
        } else {
            match btn_id {
                "back" => Some(Transition::Pop),
                _ => None,
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_mouse_button_pressed(ctx, MouseButton::X1) {
            Ok(Transition::Pop)
        } else if let Some(t) = update_sprites(self, ctx) {
            Ok(t)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.redraw_sprites(ctx)
    }

    fn sprites(&mut self) -> Option<&mut Vec<Box<dyn Sprite>>> {
        Some(&mut self.sprites)
    }
}
