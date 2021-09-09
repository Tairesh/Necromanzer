use assets::Assets;
use avatar::Avatar;
use colors::Colors;
use human::character::Character;
use human::gender::Gender;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use map::pos::TilePos;
use rand::seq::SliceRandom;
use rand::Rng;
use savefile::SaveFile;
use scenes::game::scene::Game;
use scenes::manager::{update_sprites, Scene, Transition};
use settings::Settings;
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::meshy::JustMesh;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::{Colorize, Draw, Positionate, Sprite, Stringify};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, window, Context};
use world::World;
use Vec2;

pub struct CreateCharacter {
    assets: Rc<RefCell<Assets>>,
    settings: Rc<RefCell<Settings>>,
    savefile: SaveFile,
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
    name_input: Rc<RefCell<TextInput>>,
    name_empty: Rc<RefCell<Label>>,
    gender_input: Rc<RefCell<TextInput>>,
    age_input: Rc<RefCell<TextInput>>,
    hand_label: Rc<RefCell<Label>>,
    skin_mesh: Rc<RefCell<JustMesh>>,
    skin_label: Rc<RefCell<Label>>,
    main_hand: MainHand,
    skin_tone: SkinTone,
}

impl CreateCharacter {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        settings: Rc<RefCell<Settings>>,
        savefile: SaveFile,
        ctx: &mut Context,
    ) -> Self {
        let mut sprites: Vec<Rc<RefCell<dyn Sprite>>> = Vec::new();
        sprites.push(Rc::new(RefCell::new(Image::new(
            assets.borrow().bg.clone(),
            Position::center(),
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Create new character:",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            format!("New adventurer in the «{}» world", savefile.meta.name).as_str(),
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, 100.0, AnchorY::Top),
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Name:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(194.0),
            },
        ))));
        let name_input = Rc::new(RefCell::new(TextInput::new(
            "",
            300.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        )));
        sprites.push(name_input.clone());
        let name_empty = Rc::new(RefCell::new(Label::hidden(
            "Character name shall not be empty!",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Bottom.to_position(170.0),
            },
        )));
        sprites.push(name_empty.clone());
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Gender:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(244.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "gender_left",
            vec![],
            assets.borrow().regions.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        ))));
        let gender_input = Rc::new(RefCell::new(TextInput::new(
            if savefile.time.elapsed().unwrap().as_secs() % 2 == 0 {
                "Male"
            } else {
                "Female"
            },
            210.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(gender_input.clone());
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "gender_right",
            vec![],
            assets.borrow().regions.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Age:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(294.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "age_minus",
            vec![],
            assets.borrow().regions.minus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        ))));
        let age_input = Rc::new(RefCell::new(TextInput::int(
            18,
            (16, 99),
            210.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(age_input.clone());
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "age_plus",
            vec![],
            assets.borrow().regions.plus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Main hand:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "hand_left",
            vec![],
            assets.borrow().regions.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        ))));
        let hand_label = Rc::new(RefCell::new(Label::new(
            "Right",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        )));
        sprites.push(hand_label.clone());
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "hand_right",
            vec![],
            assets.borrow().regions.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Label::new(
            "Skin tone:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "skin_left",
            vec![],
            assets.borrow().regions.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        ))));
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
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        )));
        sprites.push(skin_mesh.clone());
        let skin_label = Rc::new(RefCell::new(Label::new(
            "Warm Ivory",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        )));
        sprites.push(skin_label.clone());
        sprites.push(Rc::new(RefCell::new(Button::icon(
            "skin_right",
            vec![],
            assets.borrow().regions.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        ))));
        let mut randomize_btn = Button::new(
            "randomize",
            vec![
                (Key::NumPadMultiply, None),
                (Key::Num8, Some(KeyModifier::Shift)),
            ],
            "[*] Randomize",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenter { offset: 0.0 },
                y: AnchorY::Center.to_position(500.0),
            },
        );
        let randomize_size = randomize_btn.calc_size(ctx);
        sprites.push(Rc::new(RefCell::new(Button::new(
            "back",
            vec![(Key::Escape, None)],
            "[Esc] Back",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight {
                    offset: -randomize_size.x / 2.0 - 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        ))));
        sprites.push(Rc::new(RefCell::new(randomize_btn)));
        sprites.push(Rc::new(RefCell::new(Button::new(
            "create",
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        ))));
        Self {
            assets,
            settings,
            savefile,
            sprites,
            name_input,
            name_empty,
            gender_input,
            age_input,
            hand_label,
            skin_mesh,
            skin_label,
            main_hand: MainHand::Right,
            skin_tone: SkinTone::WarmIvory,
        }
    }
}

impl Scene for CreateCharacter {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "back" => Some(Transition::Pop),
            "create" => {
                let name = self.name_input.borrow().value();
                if name.is_empty() {
                    self.name_input.borrow_mut().set_danger(true);
                    self.name_empty.borrow_mut().set_visible(true);
                    None
                } else {
                    let gender = self.gender_input.borrow().value();
                    let age = self.age_input.borrow().value().parse::<u8>().unwrap();
                    let character = Character::new(
                        name,
                        Gender::from_string(gender),
                        age,
                        self.main_hand,
                        self.skin_tone,
                    );
                    let avatar = Avatar::new(character, TilePos::new(0, 0));
                    let mut world = World::new(
                        self.assets.clone(),
                        self.savefile.path.clone(),
                        self.savefile.meta.clone(),
                        avatar,
                        HashMap::new(),
                    )
                    .init();
                    world.save();
                    Some(Transition::Replace(Box::new(Game::new(
                        self.assets.clone(),
                        self.settings.clone(),
                        world,
                        ctx,
                    ))))
                }
            }
            "randomize" => {
                let mut gender = self.gender_input.borrow_mut();
                gender.set_value(if rand::random::<bool>() {
                    "Male"
                } else {
                    "Female"
                });
                let assets = self.assets.borrow();
                let name = *match gender.value().as_str() {
                    "Male" => &assets.male_names,
                    "Female" => &assets.female_names,
                    _ => &assets.names,
                }
                .choose(&mut rand::thread_rng())
                .unwrap();
                self.name_input.borrow_mut().set_value(name);
                self.age_input
                    .borrow_mut()
                    .set_value(format!("{}", rand::thread_rng().gen_range(16..=99)).as_str());
                self.main_hand = rand::random::<MainHand>();
                let mut hand = self.hand_label.borrow_mut();
                hand.set_value(self.main_hand.name());
                let window_size = window::get_size(ctx);
                hand.positionate(ctx, window_size);
                self.skin_tone = rand::random::<SkinTone>();
                self.skin_mesh
                    .borrow_mut()
                    .set_color(self.skin_tone.color());
                let mut label = self.skin_label.borrow_mut();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window_size);
                None
            }
            "gender_left" | "gender_right" => {
                let mut input = self.gender_input.borrow_mut();
                let value = input.value();
                input.set_value(if value == "Male" { "Female" } else { "Male" });
                None
            }
            "age_plus" | "age_minus" => {
                let mut input = self.age_input.borrow_mut();
                if let Ok(mut value) = input.value().parse::<u8>() {
                    if btn_id == "age_plus" {
                        value += 1;
                    } else if value > 0 {
                        value -= 1;
                    }
                    input.set_value(format!("{}", value).as_str());
                }
                None
            }
            "hand_left" | "hand_right" => {
                let mut label = self.hand_label.borrow_mut();
                self.main_hand = if btn_id == "hand_right" {
                    self.main_hand.next()
                } else {
                    self.main_hand.prev()
                };
                label.set_value(self.main_hand.name());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            "skin_left" | "skin_right" => {
                self.skin_tone = if btn_id == "skin_right" {
                    self.skin_tone.next()
                } else {
                    self.skin_tone.prev()
                };
                self.skin_mesh
                    .borrow_mut()
                    .set_color(self.skin_tone.color());
                let mut label = self.skin_label.borrow_mut();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            _ => None,
        }
    }

    fn update(&mut self, ctx: &mut Context) -> Option<Transition> {
        {
            let mut name_error = self.name_empty.borrow_mut();
            if !self.name_input.borrow().danger() && name_error.visible() {
                name_error.set_visible(false);
            }
        };
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
