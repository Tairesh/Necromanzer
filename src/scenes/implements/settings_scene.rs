use tetra::input::{Key, KeyModifier};
use tetra::window::WindowPosition;
use tetra::{Context, Event};

use app::App;
use scenes::scene_impl::SceneImpl;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, label, title};
use settings::Settings;
use ui::button::Button;
use ui::inputs::TextInput;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::{Positionate, Press, Stringify, UiSprite};
use ui::{SomeUISprites, SomeUISpritesMut};

const WINDOW_MODE_EVENT: u8 = 1;
const FULLSCREEN_MODE_EVENT: u8 = 2;
const REPEAT_INTERVAL_MINUS: u8 = 3;
const REPEAT_INTERVAL_PLUS: u8 = 4;

pub struct SettingsScene {
    sprites: [Box<dyn UiSprite>; 10],
}

impl SettingsScene {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(app: &App, ctx: &mut Context) -> Self {
        let bg = bg(&app.assets);
        let title = title("Settings", &app.assets);

        let settings = Settings::instance();
        let fullscreen_btn = Box::new(Button::fixed(
            vec![(Key::F, KeyModifier::Alt).into()],
            "[Alt+F] Fullscreen",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            settings.window.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 100.0 },
                y: Vertical::ByCenter { y: 150.0 },
            },
            Transition::CustomEvent(FULLSCREEN_MODE_EVENT),
        ));
        let mut window_btn = Box::new(Button::fixed(
            vec![(Key::W, KeyModifier::Alt).into()],
            "[Alt+W] Window",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            !settings.window.fullscreen,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 98.0 },
                y: Vertical::ByCenter { y: 150.0 },
            },
            Transition::CustomEvent(WINDOW_MODE_EVENT),
        ));
        let window_btn_size = window_btn.calc_size(ctx);

        let window_mode_label = label(
            "Window mode:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_btn_size.x,
                },
                y: Vertical::ByCenter { y: 145.0 },
            },
        );

        let repeat_interval_label = label(
            "Repeat delay:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: 90.0 - window_btn_size.x,
                },
                y: Vertical::ByCenter { y: 195.0 },
            },
        );
        let repeat_interval_minus = Box::new(Button::icon(
            vec![],
            "minus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 0.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
            Transition::CustomEvent(REPEAT_INTERVAL_MINUS),
        ));
        let repeat_interval_input = Box::new(TextInput::int(
            settings.repeat_interval as u32,
            (1, 10000),
            190.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
        ));
        let repeat_interval_plus = Box::new(Button::icon(
            vec![],
            "plus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 200.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
            Transition::CustomEvent(REPEAT_INTERVAL_PLUS),
        ));

        let back_btn = back_btn(
            Position::horizontal_center(0.0, Vertical::AtWindowBottomByBottom { offset: -200.0 }),
            &app.assets,
        );

        Self {
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg,
                title,
                fullscreen_btn,
                window_btn,
                window_mode_label,
                repeat_interval_label,
                repeat_interval_minus,
                repeat_interval_input,
                repeat_interval_plus,
                back_btn,
            ],
        }
    }

    fn fullscreen_btn(&mut self) -> &mut Button {
        self.sprites[2].as_button().unwrap()
    }

    fn window_btn(&mut self) -> &mut Button {
        self.sprites[3].as_button().unwrap()
    }

    fn repeat_interval_input(&mut self) -> &mut TextInput {
        self.sprites[7].as_text_input().unwrap()
    }
}

impl SceneImpl for SettingsScene {
    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, self.is_there_focused_sprite())
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        match event {
            FULLSCREEN_MODE_EVENT => {
                self.window_btn().unpress();
                if !tetra::window::is_fullscreen(ctx) {
                    Settings::instance().window.fullscreen = true;
                    tetra::window::set_fullscreen(ctx, true).ok();
                }
                None
            }
            WINDOW_MODE_EVENT => {
                self.fullscreen_btn().unpress();
                if tetra::window::is_fullscreen(ctx) {
                    Settings::instance().window.fullscreen = false;
                    tetra::window::set_fullscreen(ctx, false).ok();
                    tetra::window::set_decorated(ctx, true);
                    let window_settings = &Settings::instance().window;
                    tetra::window::set_size(
                        ctx,
                        window_settings.width as i32,
                        window_settings.height as i32,
                    )
                    .ok();
                    let current_monitor = tetra::window::get_current_monitor(ctx).unwrap_or(0);
                    tetra::window::set_position(
                        ctx,
                        WindowPosition::Centered(current_monitor),
                        WindowPosition::Centered(current_monitor),
                    );
                }
                None
            }
            REPEAT_INTERVAL_MINUS | REPEAT_INTERVAL_PLUS => {
                let input = self.repeat_interval_input();
                if let Ok(mut value) = input.value().parse::<u32>() {
                    match event {
                        REPEAT_INTERVAL_MINUS => {
                            value -= 1;
                        }
                        REPEAT_INTERVAL_PLUS => {
                            value += 1;
                        }
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{}", value).as_str());
                    Settings::instance().repeat_interval = value;
                }
                None
            }
            _ => None,
        }
    }
}

impl Drop for SettingsScene {
    fn drop(&mut self) {
        Settings::instance().save();
    }
}
