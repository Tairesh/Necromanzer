use assets::Assets;
use colors::Colors;
use human::main_hand::MainHand;
use human::skin_tone::SkinTone;
use rand::seq::SliceRandom;
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
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::{input, window, Context, TetraVec2};

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
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(194.0),
            },
        )));
        sprites.push(Box::new(TextInput::new(
            "name",
            "",
            300.0,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(200.0),
            },
        )));
        sprites.push(Box::new(Label::hidden(
            "Character name shall not be empty!",
            assets.borrow().default.clone(),
            Colors::RED,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Bottom.to_position(170.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Gender:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(244.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "gender_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
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
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "gender_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(250.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Age:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(294.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "age_minus",
            vec![],
            assets.borrow().icons.minus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
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
                x: Horizontal::AtWindowCenterByLeft { offset: 5.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "age_plus",
            vec![],
            assets.borrow().icons.plus,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(300.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Main hand:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "hand_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Left",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(344.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "hand_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
                y: AnchorY::Center.to_position(350.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Skin tone:",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: -60.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "skin_left",
            vec![],
            assets.borrow().icons.lt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByLeft { offset: -40.0 },
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
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(400.0),
            },
        )));
        sprites.push(Box::new(Label::new(
            "Warm Ivory",
            assets.borrow().header2.clone(),
            Colors::DARK_BROWN,
            Position {
                x: Horizontal::AtWindowCenter { offset: 110.0 },
                y: AnchorY::Center.to_position(394.0),
            },
        )));
        sprites.push(Box::new(Button::icon(
            "skin_right",
            vec![],
            assets.borrow().icons.mt,
            assets.clone(),
            Position {
                x: Horizontal::AtWindowCenterByRight { offset: 260.0 },
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
            vec![(Key::Enter, Some(KeyModifier::Alt))],
            "[Alt+Enter] Create",
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
            "create" => {
                if self.sprites.get(4).unwrap().get_value().unwrap().is_empty() {
                    self.sprites.get_mut(4).unwrap().set_danger(true);
                    self.sprites.get_mut(5).unwrap().set_visible(true);
                }
                None
            }
            "randomize" => {
                let gender = self.sprites.get_mut(8).unwrap();
                gender.set_value(if rand::random::<bool>() {
                    "Male"
                } else {
                    "Female"
                });
                let assets = self.assets.borrow();
                let name = *match gender.get_value().unwrap().as_str() {
                    "Male" => &assets.male_names,
                    "Female" => &assets.female_names,
                    _ => &assets.names,
                }
                .choose(&mut rand::thread_rng())
                .unwrap();
                let input = self.sprites.get_mut(4).unwrap();
                input.set_value(name);
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
        if !self.sprites.get(4).unwrap().get_danger() && self.sprites.get(5).unwrap().visible() {
            self.sprites.get_mut(5).unwrap().set_visible(false);
        }
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
