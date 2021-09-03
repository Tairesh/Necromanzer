use action::Action;
use assets::Assets;
use direction::TwoDimDirection;
use human::character::Character;
use human::gender::Gender;
use map::pos::TilePos;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::graphics::DrawParams;
use tetra::{Context, TetraVec2};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Avatar {
    pub character: Character,
    pub pos: TilePos,
    pub action: Option<Action>,
    pub vision: TwoDimDirection,
}

impl Avatar {
    pub fn new(character: Character, pos: TilePos) -> Self {
        Avatar {
            character,
            pos,
            action: None,
            vision: TwoDimDirection::East,
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        assets: Rc<RefCell<Assets>>,
        mut position: TetraVec2,
        zoom: f32,
    ) {
        let scale = if let TwoDimDirection::East = self.vision {
            TetraVec2::new(zoom, zoom)
        } else {
            position.x += 10.0 * zoom;
            TetraVec2::new(-zoom, zoom)
        };
        let assets = assets.borrow();
        assets.tileset.draw_region(
            ctx,
            match self.character.gender {
                Gender::Female => assets.regions.female,
                Gender::Male => assets.regions.male,
                Gender::Custom(_) => assets.regions.queer,
            },
            DrawParams::new()
                .position(position)
                .scale(scale)
                .color(self.character.skin_tone.color()),
        );
    }
}
