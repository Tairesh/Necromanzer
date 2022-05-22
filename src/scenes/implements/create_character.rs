use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;

use num_enum::{IntoPrimitive, TryFromPrimitive};
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};
use variant_count::VariantCount;

use app::App;
use colors::Colors;
use cycle_enum::CycleEnum;
use game::bodies::BodySize;
use game::human::hair_color::HairColor;
use game::human::main_hand::MainHand;
use game::human::personality::{Appearance, Mind, Personality};
use game::human::skin_tone::SkinTone;
use game::map::pos::TilePos;
use game::{Avatar, Log, World};
use savefile;
use savefile::{GameView, Meta};
use scenes::scene::Scene;
use scenes::scene_impl::SceneImpl;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, error_label, label, title};
use ui::button::Button;
use ui::inputs::TextInput;
use ui::label::Label;
use ui::position::{Horizontal, Position, Vertical};
use ui::traits::{Draw, Positionate, Stringify, UiSprite};
use ui::{SomeUISprites, SomeUISpritesMut};

#[derive(IntoPrimitive, TryFromPrimitive, VariantCount, Debug, Copy, Clone)]
#[repr(u8)]
enum Events {
    GenderLeft,
    GenderRight,
    AgeMinus,
    AgePlus,
    HandLeft,
    HandRight,
    Randomize,
    Create,
}

pub struct CreateCharacter {
    meta: Meta,
    sprites: [Box<dyn UiSprite>; 21],
    main_hand: MainHand,
    window_size: (i32, i32),
}

impl CreateCharacter {
    // TODO: refactor and delete this allow
    #[allow(clippy::too_many_lines)]
    pub fn new(path: &Path, app: &App, ctx: &mut Context) -> Self {
        let meta = savefile::load(path).unwrap();
        let bg = bg(&app.assets);
        let title = title("Create new character:", &app.assets);
        let subtitle = Box::new(Label::new(
            format!("New adventurer in the «{}» world", meta.name),
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 100.0 }),
        ));

        let name_label = label(
            "Name:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 195.0 },
            },
        );
        let name_input = Box::new(TextInput::new(
            "",
            300.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
        ));
        let name_empty = error_label(
            "Character name shall not be empty!",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByBottom { y: 170.0 },
            },
        );
        let gender_label = label(
            "Gender:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 245.0 },
            },
        );
        let gender_left = Box::new(Button::icon(
            vec![],
            "lt",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 250.0 },
            },
            Transition::CustomEvent(Events::GenderLeft as u8),
        ));
        let gender_input = Box::new(TextInput::new(
            if meta.time.elapsed().unwrap().as_secs() % 2 == 0 {
                "Female"
            } else {
                "Male"
            },
            210.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::ByCenter { y: 250.0 },
            },
        ));
        let gender_right = Box::new(Button::icon(
            vec![],
            "mt",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 250.0 },
            },
            Transition::CustomEvent(Events::GenderRight as u8),
        ));
        let age_label = label(
            "Age:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 298.0 },
            },
        );
        let age_minus = Box::new(Button::icon(
            vec![],
            "minus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
            Transition::CustomEvent(Events::AgeMinus as u8),
        ));
        let age_input = Box::new(TextInput::int(
            18,
            (16, 99),
            210.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
        ));
        let age_plus = Box::new(Button::icon(
            vec![],
            "plus",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
            Transition::CustomEvent(Events::AgePlus as u8),
        ));
        let hand_label = label(
            "Main hand:",
            &app.assets,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 345.0 },
            },
        );
        let hand_left = Box::new(Button::icon(
            vec![],
            "lt",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 350.0 },
            },
            Transition::CustomEvent(Events::HandLeft as u8),
        ));
        let hand_name = Box::new(Label::new(
            "Right",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByCenter { y: 348.0 },
            },
        ));
        let hand_right = Box::new(Button::icon(
            vec![],
            "mt",
            app.assets.tileset.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 350.0 },
            },
            Transition::CustomEvent(Events::HandRight as u8),
        ));

        let mut randomize_btn = Box::new(Button::text(
            vec![
                Key::NumPadMultiply.into(),
                (Key::Num8, KeyModifier::Shift).into(),
            ],
            "[*] Randomize",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(Events::Randomize as u8),
        ));
        let randomize_btn_size = randomize_btn.calc_size(ctx);
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_btn_size.x / 2.0 - 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            &app.assets,
        );
        let create_btn = Box::new(Button::text(
            vec![(Key::Enter, KeyModifier::Alt).into()],
            "[Alt+Enter] Create",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_btn_size.x / 2.0 + 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(Events::Create as u8),
        ));

        Self {
            meta,
            // Order is matter, change hardcoded indices in functions below if modified
            sprites: [
                bg,
                title,
                subtitle,
                name_label,
                name_input,
                name_empty,
                gender_label,
                gender_left,
                gender_input,
                gender_right,
                age_label,
                age_minus,
                age_input,
                age_plus,
                hand_label,
                hand_left,
                hand_name,
                hand_right,
                back_btn,
                randomize_btn,
                create_btn,
            ],
            main_hand: MainHand::Right,
            window_size: app.window_size,
        }
    }

    fn name_input(&mut self) -> &mut TextInput {
        self.sprites[4].as_text_input().unwrap()
    }
    fn name_empty(&mut self) -> &mut Label {
        self.sprites[5].as_label().unwrap()
    }
    fn gender_input(&mut self) -> &mut TextInput {
        self.sprites[8].as_text_input().unwrap()
    }
    fn age_input(&mut self) -> &mut TextInput {
        self.sprites[12].as_text_input().unwrap()
    }
    fn hand_name(&mut self) -> &mut Label {
        self.sprites[16].as_label().unwrap()
    }
}

impl SceneImpl for CreateCharacter {
    fn on_update(&mut self, _ctx: &mut Context) -> SomeTransitions {
        if !self.name_input().danger() && self.name_empty().visible() {
            self.name_empty().set_visible(false);
        }
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(&event, self.is_there_focused_sprite())
    }

    fn on_resize(&mut self, _ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
    }

    fn sprites(&self) -> SomeUISprites {
        Some(&self.sprites)
    }

    fn sprites_mut(&mut self) -> SomeUISpritesMut {
        Some(&mut self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        // TODO: remove unwrap
        let event = Events::try_from(event).unwrap();
        match event {
            Events::GenderLeft | Events::GenderRight => {
                let input = self.gender_input();
                let value = input.value();
                input.set_value(if value == "Male" { "Female" } else { "Male" });
                None
            }
            Events::AgeMinus | Events::AgePlus => {
                let input = self.age_input();
                if let Ok(mut value) = input.value().parse::<u8>() {
                    match event {
                        Events::AgeMinus => {
                            value -= 1;
                        }
                        Events::AgePlus => {
                            value += 1;
                        }
                        _ => unreachable!(),
                    }
                    input.set_value(format!("{}", value).as_str());
                }
                None
            }
            Events::HandLeft | Events::HandRight => {
                self.main_hand = match event {
                    Events::HandRight => self.main_hand.next(),
                    Events::HandLeft => self.main_hand.prev(),
                    _ => unreachable!(),
                };
                let name = self.main_hand.name();
                let window_size = self.window_size;
                self.hand_name().update(name, ctx, window_size);
                None
            }
            Events::Randomize => {
                let mut rng = rand::thread_rng();
                let character = Personality::random(&mut rng, true);
                self.gender_input().set_value(character.mind.gender);
                self.name_input().set_value(character.mind.name);
                self.age_input()
                    .set_value(character.appearance.age.to_string());
                self.main_hand = character.mind.main_hand;
                let name = self.main_hand.name();
                let window_size = self.window_size;
                self.hand_name().update(name, ctx, window_size);
                None
            }
            Events::Create => {
                let name = self.name_input().value();
                if name.is_empty() {
                    self.name_input().set_danger(true);
                    self.name_empty().set_visible(true);
                    None
                } else {
                    let gender = self.gender_input().value().into();
                    let age = self.age_input().value().parse::<u8>().unwrap();
                    let character = Personality::new(
                        Appearance {
                            age,
                            skin_tone: SkinTone::PaleIvory,
                            hair_color: HairColor::White,
                            body_size: BodySize::Normal,
                        },
                        Mind {
                            name,
                            gender,
                            main_hand: self.main_hand,
                            alive: true,
                        },
                    );
                    // TODO: find available starting pos in the world
                    let avatar = Avatar::player(character, TilePos::new(0, 0));
                    let mut world = World::new(
                        self.meta.clone(),
                        GameView::default(),
                        Log::new(),
                        vec![avatar],
                        HashMap::new(),
                    )
                    .init();
                    world.save();
                    Some(vec![
                        Transition::LoadWorld(self.meta.path.clone()),
                        Transition::Replace(Scene::GameScene),
                    ])
                }
            }
        }
    }
}
