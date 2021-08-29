use assets::Assets;
use colors::Colors;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use savefile::SaveFile;
use scenes::manager::{update_sprites, Scene, Transition};
use sprites::button::Button;
use sprites::image::Image;
use sprites::input::TextInput;
use sprites::label::Label;
use sprites::meshy::JustMesh;
use sprites::position::{AnchorY, Horizontal, Position};
use sprites::sprite::{Positionate, Sprite};
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::{Color, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, window, Context, TetraVec2};

enum MainHand {
    Left,
    Right,
    Ambidexter,
}

impl MainHand {
    pub fn name(&self) -> &str {
        match self {
            MainHand::Left => "Left",
            MainHand::Right => "Right",
            MainHand::Ambidexter => "Ambidexter",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            MainHand::Left => MainHand::Right,
            MainHand::Right => MainHand::Ambidexter,
            MainHand::Ambidexter => MainHand::Left,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            MainHand::Left => MainHand::Ambidexter,
            MainHand::Right => MainHand::Left,
            MainHand::Ambidexter => MainHand::Right,
        }
    }
}

impl Distribution<MainHand> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MainHand {
        if rng.gen_bool(0.01) {
            MainHand::Ambidexter
        } else if rng.gen_bool(0.16) {
            MainHand::Left
        } else {
            MainHand::Right
        }
    }
}

enum SkinTone {
    PaleIvory,
    WarmIvory,
    Sand,
    RoseBeige,
    Sienna,
    Limestone,
    Beige,
    Amber,
    Honey,
    Band,
    Almond,
    Umber,
    Bronze,
    Golden,
    Espresso,
    Chocolate,
}

impl SkinTone {
    pub fn name(&self) -> &str {
        match self {
            SkinTone::PaleIvory => "Pale Ivory",
            SkinTone::WarmIvory => "Warm Ivory",
            SkinTone::Sand => "Sandy",
            SkinTone::RoseBeige => "Rose Beige",
            SkinTone::Sienna => "Sienna",
            SkinTone::Limestone => "Limestone",
            SkinTone::Beige => "Beige",
            SkinTone::Amber => "Amber",
            SkinTone::Honey => "Honey",
            SkinTone::Band => "Band",
            SkinTone::Almond => "Almond",
            SkinTone::Umber => "Umber",
            SkinTone::Bronze => "Bronze",
            SkinTone::Golden => "Golden",
            SkinTone::Espresso => "Espresso",
            SkinTone::Chocolate => "Chocolate",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SkinTone::PaleIvory => SkinTone::WarmIvory,
            SkinTone::WarmIvory => SkinTone::Sand,
            SkinTone::Sand => SkinTone::RoseBeige,
            SkinTone::RoseBeige => SkinTone::Sienna,
            SkinTone::Sienna => SkinTone::Limestone,
            SkinTone::Limestone => SkinTone::Beige,
            SkinTone::Beige => SkinTone::Amber,
            SkinTone::Amber => SkinTone::Honey,
            SkinTone::Honey => SkinTone::Band,
            SkinTone::Band => SkinTone::Almond,
            SkinTone::Almond => SkinTone::Umber,
            SkinTone::Umber => SkinTone::Bronze,
            SkinTone::Bronze => SkinTone::Golden,
            SkinTone::Golden => SkinTone::Espresso,
            SkinTone::Espresso => SkinTone::Chocolate,
            SkinTone::Chocolate => SkinTone::PaleIvory,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            SkinTone::PaleIvory => SkinTone::Chocolate,
            SkinTone::WarmIvory => SkinTone::PaleIvory,
            SkinTone::Sand => SkinTone::WarmIvory,
            SkinTone::RoseBeige => SkinTone::Sand,
            SkinTone::Sienna => SkinTone::RoseBeige,
            SkinTone::Limestone => SkinTone::Sienna,
            SkinTone::Beige => SkinTone::Limestone,
            SkinTone::Amber => SkinTone::Beige,
            SkinTone::Honey => SkinTone::Amber,
            SkinTone::Band => SkinTone::Honey,
            SkinTone::Almond => SkinTone::Band,
            SkinTone::Umber => SkinTone::Almond,
            SkinTone::Bronze => SkinTone::Umber,
            SkinTone::Golden => SkinTone::Bronze,
            SkinTone::Espresso => SkinTone::Golden,
            SkinTone::Chocolate => SkinTone::Espresso,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            SkinTone::PaleIvory => Colors::PALE_IVORY,
            SkinTone::WarmIvory => Colors::WARM_IVORY,
            SkinTone::Sand => Colors::SAND,
            SkinTone::RoseBeige => Colors::ROSE_BEIGE,
            SkinTone::Sienna => Colors::SIENNA,
            SkinTone::Limestone => Colors::LIMESTONE,
            SkinTone::Beige => Colors::BEIGE,
            SkinTone::Amber => Colors::AMBER,
            SkinTone::Honey => Colors::HONEY,
            SkinTone::Band => Colors::BAND,
            SkinTone::Almond => Colors::ALMOND,
            SkinTone::Umber => Colors::UMBER,
            SkinTone::Bronze => Colors::BRONZE,
            SkinTone::Golden => Colors::GOLDEN,
            SkinTone::Espresso => Colors::ESPRESSO,
            SkinTone::Chocolate => Colors::CHOCOLATE,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            SkinTone::Almond
            | SkinTone::Umber
            | SkinTone::Bronze
            | SkinTone::Golden
            | SkinTone::Espresso
            | SkinTone::Chocolate => Colors::LIGHT_YELLOW,
            _ => Colors::DARK_BROWN,
        }
    }
}

impl Distribution<SkinTone> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinTone {
        match rng.gen_range(0..=15) {
            0 => SkinTone::PaleIvory,
            1 => SkinTone::WarmIvory,
            2 => SkinTone::Sand,
            3 => SkinTone::RoseBeige,
            4 => SkinTone::Sienna,
            5 => SkinTone::Limestone,
            6 => SkinTone::Beige,
            7 => SkinTone::Amber,
            8 => SkinTone::Honey,
            9 => SkinTone::Band,
            10 => SkinTone::Almond,
            11 => SkinTone::Umber,
            12 => SkinTone::Bronze,
            13 => SkinTone::Golden,
            14 => SkinTone::Espresso,
            15 => SkinTone::Chocolate,
            _ => {
                panic!("Rust is the memory safe language with zero cost abstractions!");
            }
        }
    }
}

#[allow(dead_code)]
pub struct CreateCharacter {
    assets: Rc<RefCell<Assets>>,
    sprites: Vec<Box<dyn Sprite>>,
    main_hand: MainHand,
    skin_tone: SkinTone,
    savefile: SaveFile,
}

impl CreateCharacter {
    pub fn new(assets: Rc<RefCell<Assets>>, savefile: SaveFile, ctx: &mut Context) -> Self {
        let mut sprites: Vec<Box<dyn Sprite>> = Vec::new();
        sprites.push(Box::new(Image::new(
            assets.borrow().bg.clone(),
            Position::center(),
        )));
        sprites.push(Box::new(Label::new(
            "Create new character:",
            assets.borrow().header1.clone(),
            Colors::DARK_GREEN,
            Position::horizontal_center(0.0, 20.0, AnchorY::Top),
        )));
        sprites.push(Box::new(Label::new(
            format!("New adventurer in the «{}» world", savefile.name).as_str(),
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position::horizontal_center(0.0, 100.0, AnchorY::Top),
        )));
        sprites.push(Box::new(Label::new(
            "Name:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(194.0),
            },
        )));
        sprites.push(Box::new(TextInput::new(
            "name",
            "",
            300.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        )));
        sprites.push(Box::new(Label::hidden(
            "Character with this name already exists",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Bottom.to_position(180.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Gender:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(244.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "gender_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(Box::new(TextInput::new(
            "gender",
            if savefile.time.elapsed().unwrap().as_secs() % 2 == 0 {
                "Male"
            } else {
                "Female"
            },
            210.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 45.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "gender_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 300.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Age:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(294.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "age_minus",
            vec![],
            assets.borrow().icons.minus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(Box::new(TextInput::int(
            "age",
            18,
            (16, 99),
            210.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 45.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "age_plus",
            vec![],
            assets.borrow().icons.plus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 300.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Main hand:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "hand_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Left",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 150.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "hand_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 300.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Skin tone:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -10.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "skin_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: 0.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        )));
        sprites.push(Box::new(JustMesh::new(
            Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, 210.0, 42.0),
                BorderRadii::new(10.0),
            )
            .unwrap(),
            Some(Colors::WARM_IVORY),
            TetraVec2::new(210.0, 42.0),
            Position {
                x: Horizontal::AtWindowCenter { offset: 150.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Warm Ivory",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 150.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "skin_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 300.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        )));
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
        sprites.push(Box::new(Button::new(
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
        )));
        sprites.push(Box::new(randomize_btn));
        sprites.push(Box::new(Button::new(
            "create",
            vec![(Key::Enter, None)],
            "[Enter] Create",
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft {
                    offset: randomize_size.x / 2.0 + 2.0,
                },
                y: AnchorY::Center.to_position(500.0),
            },
        )));
        Self {
            assets,
            sprites,
            savefile,
            main_hand: MainHand::Left,
            skin_tone: SkinTone::WarmIvory,
        }
    }
}

impl Scene for CreateCharacter {
    fn on_button_click(&mut self, ctx: &mut Context, btn_id: &str) -> Option<Transition> {
        match btn_id {
            "back" => Some(Transition::Pop),
            "randomize" => {
                let gender = self.sprites.get_mut(8).unwrap();
                gender.set_value(if rand::random::<bool>() {
                    "Male"
                } else {
                    "Female"
                });
                let age = self.sprites.get_mut(12).unwrap();
                age.set_value(format!("{}", rand::thread_rng().gen_range(16..=99)).as_str());
                self.main_hand = rand::random::<MainHand>();
                let hand = self.sprites.get_mut(16).unwrap();
                hand.set_value(self.main_hand.name());
                hand.positionate(ctx, window::get_size(ctx));
                self.skin_tone = rand::random::<SkinTone>();
                let mesh = self.sprites.get_mut(20).unwrap();
                mesh.set_color(self.skin_tone.color());
                let label = self.sprites.get_mut(21).unwrap();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            "gender_left" | "gender_right" => {
                let input = self.sprites.get_mut(8).unwrap();
                if let Some(value) = input.get_value() {
                    input.set_value(if value == "Male" { "Female" } else { "Male" });
                }
                None
            }
            "age_plus" | "age_minus" => {
                let input = self.sprites.get_mut(12).unwrap();
                if let Some(value) = input.get_value() {
                    if let Ok(mut value) = value.parse::<u32>() {
                        if btn_id == "age_plus" {
                            value += 1;
                        } else if value > 0 {
                            value -= 1;
                        }
                        input.set_value(format!("{}", value).as_str());
                    }
                }
                None
            }
            "hand_left" | "hand_right" => {
                let label = self.sprites.get_mut(16).unwrap();
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
                let mesh = self.sprites.get_mut(20).unwrap();
                mesh.set_color(self.skin_tone.color());
                let label = self.sprites.get_mut(21).unwrap();
                label.set_value(self.skin_tone.name());
                label.set_color(self.skin_tone.text_color());
                label.positionate(ctx, window::get_size(ctx));
                None
            }
            _ => None,
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

    fn sprites(&mut self) -> Option<&mut Vec<Box<dyn Sprite>>> {
        Some(&mut self.sprites)
    }
}
