use action::Action;
use assets::Assets;
use direction::{Direction, TwoDimDirection};
use human::character::Character;
use maptile::TilePos;
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

    pub fn move_to(&mut self, dir: Direction, is_teleport: bool) {
        self.pos = self.pos.add(dir);
        if !is_teleport {
            if let Some(dir) = dir.as_two_dimensional() {
                self.vision = dir;
            }
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
            match self.character.gender.as_str() {
                "Female" => assets.icons.female,
                "Male" => assets.icons.male,
                _ => assets.icons.queer,
            },
            DrawParams::new()
                .position(position)
                .scale(scale)
                .color(self.character.skin_tone.color()),
        );
    }
}