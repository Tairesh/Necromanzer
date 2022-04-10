use app::App;
use assets::game_data::GameData;
use avatar::Avatar;
use colors::Colors;
use geometry::Vec2;
use human::character::Character;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use map::pos::TilePos;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use savefile::Meta;
use scenes::game_scene::GameScene;
use scenes::scene::Scene;
use scenes::transition::{SomeTransitions, Transition};
use scenes::{back_btn, bg, easy_back, title};
use sprites::button::Button;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::meshy::JustMesh;
use sprites::position::{Horizontal, Position, Vertical};
use sprites::sprite::{Colorize, Draw, Positionate, Stringify};
use sprites::{BunchOfSprites, SomeSprites};
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier};
use tetra::{Context, Event};
use variant_count::VariantCount;
use world::World;

#[derive(IntoPrimitive, TryFromPrimitive, VariantCount, Debug, Copy, Clone)]
#[repr(u8)]
enum Events {
    GenderLeft,
    GenderRight,
    AgeMinus,
    AgePlus,
    HandLeft,
    HandRight,
    SkinLeft,
    SkinRight,
    Randomize,
    Create,
}

pub struct CreateCharacter {
    game_data: Rc<GameData>,
    meta: Meta,
    sprites: BunchOfSprites,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    gender_input: Rc<RefCell<TextInput>>,
    age_input: Rc<RefCell<TextInput>>,
    hand_name: Rc<RefCell<Label>>,
    skin_mesh: Rc<RefCell<JustMesh>>,
    skin_name: Rc<RefCell<Label>>,
    main_hand: MainHand,
    skin_tone: SkinTone,
}

impl CreateCharacter {
    pub fn new(meta: Meta, app: &App, ctx: &mut Context) -> Self {
        let bg = bg(&app.assets, &app.settings);
        let title = title("Create new character:", &app.assets);
        let subtitle = Rc::new(RefCell::new(Label::new(
            format!("New adventurer in the «{}» world", meta.name),
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, Vertical::ByTop { y: 100.0 }),
        )));

        let name_label = Rc::new(RefCell::new(Label::new(
            "Name:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 195.0 },
            },
        )));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            "",
            300.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 200.0 },
            },
        )));
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "Character name shall not be empty!",
            app.assets.fonts.default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByBottom { y: 170.0 },
            },
        )));
        let gender_label = Rc::new(RefCell::new(Label::new(
            "Gender:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 245.0 },
            },
        )));
        let gender_left = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.lt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 250.0 },
            },
            Transition::CustomEvent(Events::GenderLeft as u8),
        )));
        let gender_input = Rc::new(RefCell::new(TextInput::new(
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
        )));
        let gender_right = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.mt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 250.0 },
            },
            Transition::CustomEvent(Events::GenderRight as u8),
        )));
        let age_label = Rc::new(RefCell::new(Label::new(
            "Age:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 298.0 },
            },
        )));
        let age_minus = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.minus,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
            Transition::CustomEvent(Events::AgeMinus as u8),
        )));
        let age_input = Rc::new(RefCell::new(TextInput::int(
            18,
            (16, 99),
            210.0,
            app.assets.fonts.header2.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
        )));
        let age_plus = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.plus,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 300.0 },
            },
            Transition::CustomEvent(Events::AgePlus as u8),
        )));
        let hand_label = Rc::new(RefCell::new(Label::new(
            "Main hand:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 345.0 },
            },
        )));
        let hand_left = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.lt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 350.0 },
            },
            Transition::CustomEvent(Events::HandLeft as u8),
        )));
        let hand_name = Rc::new(RefCell::new(Label::new(
            "Right",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByCenter { y: 348.0 },
            },
        )));
        let hand_right = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.mt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 350.0 },
            },
            Transition::CustomEvent(Events::HandRight as u8),
        )));
        let skin_label = Rc::new(RefCell::new(Label::new(
            "Skin tone:",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: Vertical::ByCenter { y: 395.0 },
            },
        )));
        let skin_left = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.lt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: Vertical::ByCenter { y: 400.0 },
            },
            Transition::CustomEvent(Events::SkinLeft as u8),
        )));
        let skin_mesh = Rc::new(RefCell::new(JustMesh::new(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, 210.0, 42.0),
                BorderRadii::new(10.0),
            )
            .unwrap(),
            Some(Colors::WARM_IVORY),
            Vec2::new(210.0, 42.0),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByCenter { y: 400.0 },
            },
        )));
        let skin_name = Rc::new(RefCell::new(Label::new(
            "Warm Ivory",
            app.assets.fonts.header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 110.0 },
                y: Vertical::ByCenter { y: 398.0 },
            },
        )));
        let skin_right = Rc::new(RefCell::new(Button::icon(
            vec![],
            app.assets.tileset.mt,
            app.assets.tileset.texture.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: Vertical::ByCenter { y: 400.0 },
            },
            Transition::CustomEvent(Events::SkinRight as u8),
        )));

        let randomize_btn = Rc::new(RefCell::new(Button::text(
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            app.assets.fonts.default.clone(),
            app.assets.button.clone(),
            Position {
                x: Horizontal::AtWindowCenterByCenter { offset: 0.0 },
                y: Vertical::ByCenter { y: 500.0 },
            },
            Transition::CustomEvent(Events::Randomize as u8),
        )));
        let randomize_btn_size = randomize_btn.borrow_mut().calc_size(ctx);
        let back_btn = back_btn(
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_btn_size.x / 2.0 - 2.0,
                },
                y: Vertical::ByCenter { y: 500.0 },
            },
            &app.assets,
        );
        let create_btn = Rc::new(RefCell::new(Button::text(
            vec![(Key::Enter, Some(KeyModifier::Alt))],
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
        )));

        Self {
            game_data: app.assets.game_data.clone(),
            meta,
            sprites: vec![
                bg,
                title,
                subtitle,
                name_label,
                name_input.clone(),
                name_empty.clone(),
                gender_label,
                gender_left,
                gender_input.clone(),
                gender_right,
                age_label,
                age_minus,
                age_input.clone(),
                age_plus,
                hand_label,
                hand_left,
                hand_name.clone(),
                hand_right,
                skin_label,
                skin_left,
                skin_mesh.clone(),
                skin_name.clone(),
                skin_right,
                back_btn,
                randomize_btn,
                create_btn,
            ],
            name_input,
            name_empty,
            gender_input,
            age_input,
            hand_name,
            skin_mesh,
            skin_name,
            main_hand: MainHand::Right,
            skin_tone: SkinTone::WarmIvory,
        }
    }
}

impl Scene for CreateCharacter {
    fn update(&mut self, _ctx: &mut Context) -> SomeTransitions {
        let mut name_error = self.name_empty.borrow_mut();
        if !self.name_input.borrow().danger() && name_error.visible() {
            name_error.set_visible(false);
        }
        None
    }

    fn event(&mut self, _ctx: &mut Context, event: Event) -> SomeTransitions {
        easy_back(event, self.is_there_focused_sprite())
    }

    fn sprites(&self) -> SomeSprites {
        Some(&self.sprites)
    }

    fn custom_event(&mut self, ctx: &mut Context, event: u8) -> SomeTransitions {
        let event = Events::try_from(event).unwrap();
        match event {
            Events::GenderLeft | Events::GenderRight => {
                let mut input = self.gender_input.borrow_mut();
                let value = input.value();
                input.set_value(if value == "Male" { "Female" } else { "Male" });
                None
            }
            Events::AgeMinus | Events::AgePlus => {
                let mut input = self.age_input.borrow_mut();
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
                let mut label = self.hand_name.borrow_mut();
                self.main_hand = match event {
                    Events::HandRight => self.main_hand.next(),
                    Events::HandLeft => self.main_hand.prev(),
                    _ => unreachable!(),
                };
                label.set_value(self.main_hand.name());
                label.positionate(ctx, tetra::window::get_size(ctx));
                None
            }
            Events::SkinLeft | Events::SkinRight => {
                self.skin_tone = match event {
                    Events::SkinRight => self.skin_tone.next(),
                    Events::SkinLeft => self.skin_tone.prev(),
                    _ => unreachable!(),
                };
                self.skin_mesh.borrow_mut().set_color(self.skin_tone);
                let mut label = self.skin_name.borrow_mut();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, tetra::window::get_size(ctx));
                None
            }
            Events::Randomize => {
                let mut rng = rand::thread_rng();
                let character = Character::random(&mut rng, &self.game_data);
                self.gender_input.borrow_mut().set_value(character.gender);
                self.name_input.borrow_mut().set_value(character.name);
                self.age_input
                    .borrow_mut()
                    .set_value(character.age.to_string());
                self.main_hand = character.main_hand;
                let mut hand = self.hand_name.borrow_mut();
                hand.set_value(self.main_hand.name());
                let window_size = tetra::window::get_size(ctx);
                hand.positionate(ctx, window_size);
                self.skin_tone = character.skin_tone;
                self.skin_mesh.borrow_mut().set_color(self.skin_tone);
                let mut label = self.skin_name.borrow_mut();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window_size);
                None
            }
            Events::Create => {
                let name = self.name_input.borrow().value();
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                    None
                } else {
                    let gender = self.gender_input.borrow().value().into();
                    let age = self.age_input.borrow().value().parse::<u8>().unwrap();
                    let character =
                        Character::new(name, gender, age, self.main_hand, self.skin_tone);
                    // TODO: find available starting pos in the world
                    let avatar = Avatar::new(character, TilePos::new(0, 0));
                    let mut world = World::new(
                        self.meta.clone(),
                        avatar,
                        HashMap::new(),
                        self.game_data.clone(),
                    )
                    .init();
                    world.save();
                    Some(vec![Transition::Replace(GameScene::Empty)])
                }
            }
        }
    }
}
