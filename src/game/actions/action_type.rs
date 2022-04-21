#![allow(dead_code)]

use game::actions::action::owner;
use game::World;
use geometry::direction::Direction;
use map::item::ItemType;
use map::Passage;

pub enum ActionPossibility {
    Yes,
    No(String),
}

use self::ActionPossibility::{No, Yes};

#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    SkippingTime,
    Walking(Direction),
    Wielding(Direction),
    Dropping(usize, Direction),
    Digging(Direction),
    Reading(Direction), // TODO: write test for reading
    Animate(Direction), // TODO: write test for animate
}

// TODO: get rid of all these unwraps
impl ActionType {
    pub fn length(&self, owner_id: usize, world: &World) -> u32 {
        match self {
            ActionType::SkippingTime => 1,
            ActionType::Walking(dir) => {
                // TODO: check avatar perks for calculating speed
                let pos = owner(owner_id, world).pos + dir;
                match world.get_tile(pos).unwrap().terrain.pass() {
                    Passage::Passable(length) => length.round() as u32,
                    Passage::Unpassable => 0,
                }
            }
            ActionType::Wielding(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(item) = world
                    .get_tile(pos)
                    .unwrap()
                    .items
                    .last()
                    .map(|i| i.item_type.clone())
                {
                    item.wield_time(owner(owner_id, world)).round() as u32
                } else {
                    0
                }
            }
            ActionType::Dropping(i, dir) => {
                if let Some(item) = owner(owner_id, world).wield.get(*i) {
                    let k = if matches!(dir, Direction::Here) {
                        1.0
                    } else {
                        1.5
                    };
                    (item.item_type.drop_time() * k).round() as u32
                } else {
                    0
                }
            }
            ActionType::Digging(_) => {
                // TODO: check tool quality, avatar perks and tile terrain
                1000
            }
            ActionType::Reading(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return tile.read().len() as u32;
                    }
                }

                0
            }
            ActionType::Animate(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile
                        .items
                        .iter()
                        .any(|i| matches!(i.item_type, ItemType::Corpse(..)))
                    {
                        return 5000; // TODO: use coprse mass
                    }
                }

                0
            }
        }
    }

    pub fn is_possible(&self, owner_id: usize, world: &World) -> ActionPossibility {
        match self {
            ActionType::SkippingTime => Yes,
            ActionType::Walking(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                let tile = world.get_tile(pos).unwrap();
                if !tile.terrain.is_walkable() {
                    return No(format!("You can't walk to the {}", tile.terrain.name()));
                }
                if !tile.units.is_empty() {
                    let i = tile.units.iter().copied().next().unwrap();
                    return No(format!(
                        "{} is on the way",
                        world.units.get(i).unwrap().character.name
                    ));
                }
                Yes
            }
            ActionType::Wielding(dir) => {
                if !owner(owner_id, world).wield.is_empty() {
                    return No("You already have something in your hands".to_string());
                }
                let pos = owner(owner_id, world).pos + dir;
                if world.get_tile(pos).unwrap().items.is_empty() {
                    return No("There is nothing to pick up".to_string());
                }
                Yes
            }
            ActionType::Dropping(_, dir) => {
                if owner(owner_id, world).wield.is_empty() {
                    return No("You have nothing to drop".to_string());
                }
                let pos = owner(owner_id, world).pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_walkable() {
                    return No(format!("You can't put items on {}", terrain.name()));
                }
                Yes
            }
            ActionType::Digging(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                let terrain = &world.get_tile(pos).unwrap().terrain;
                if !terrain.is_diggable() {
                    return No(format!("You can't dig the {}", terrain.name()));
                }
                if !owner(owner_id, world)
                    .wield
                    .iter()
                    .any(|i| matches!(i.item_type, ItemType::Shovel))
                {
                    return No("You have no shovel to dig!".to_string());
                }
                Yes
            }
            ActionType::Reading(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                // TODO: check skill of reading, and probably even another languages
                if let Some(tile) = world.get_tile(pos) {
                    if tile.is_readable() {
                        return Yes;
                    }
                }

                No("There is nothing to read".to_string())
            }
            ActionType::Animate(dir) => {
                let pos = owner(owner_id, world).pos + dir;
                if let Some(tile) = world.get_tile(pos) {
                    if tile
                        .items
                        .iter()
                        .any(|i| matches!(i.item_type, ItemType::Corpse(..)))
                    {
                        return Yes;
                    }
                }

                No("There is nothing to rise".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use game::actions::{Action, ActionType};
    use game::world::tests::prepare_world;
    use geometry::direction::{Direction, DIR8};
    use human::body::Freshness;
    use human::character::Character;
    use human::gender::Gender;
    use human::main_hand::MainHand;
    use human::skin_tone::SkinTone;
    use map::item::{Item, ItemType};
    use map::pos::TilePos;
    use map::terrains::{DirtVariant, GraveData, GraveVariant, Terrain};

    #[test]
    fn test_walking() {
        // TODO: add checks for failing to move to impassable terrains and units
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Terrain::Dirt(DirtVariant::Dirt3);

        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(0, world.meta.current_tick);

        let typ = ActionType::Walking(Direction::East);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(1, 0), world.player().pos);
    }

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).items.clear();
        world
            .load_tile_mut(TilePos::new(1, 0))
            .items
            .push(Item::new(ItemType::Axe));

        assert!(world.player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        let typ = ActionType::Wielding(Direction::East);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(1, world.player().wield.len());
        let item = world.player().wield.first().unwrap();
        assert!(matches!(item.item_type, ItemType::Axe));
    }

    #[test]
    fn test_skipping_time() {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        let typ = ActionType::SkippingTime;
        let length = typ.length(0, &world);
        assert_eq!(1, length);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();
        assert_eq!(1, world.meta.current_tick);
    }

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(0, 0)).terrain = Terrain::Dirt(DirtVariant::Dirt3);
        world.load_tile_mut(TilePos::new(0, 0)).items.clear();
        world.player_mut().wield.clear();
        world.player_mut().wield.push(Item::new(ItemType::Axe));

        let typ = ActionType::Dropping(0, Direction::Here);
        let length = typ.length(0, &world);
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(0, world.player().wield.len());
        assert_eq!(1, world.load_tile(TilePos::new(0, 0)).items.len());
        let item = world.load_tile(TilePos::new(0, 0)).items.first().unwrap();
        assert!(matches!(item.item_type, ItemType::Axe));
    }

    #[test]
    fn test_digging() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Terrain::Dirt(DirtVariant::Dirt3);

        let typ = ActionType::Digging(Direction::East);
        let length = typ.length(0, &world);
        assert!(Action::new(0, typ, &world).is_err());

        world.player_mut().wield.push(Item::new(ItemType::Shovel));
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit
        ));

        let character = Character::new("test", Gender::Male, 25, MainHand::Right, SkinTone::Amber);
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Terrain::Grave(
            GraveVariant::New,
            GraveData {
                character,
                death_year: 255,
            },
        );
        world.player_mut().action = Some(Action::new(0, typ, &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit
        ));
        let mut corpse = None;
        let mut gravestone = None;
        for dir in DIR8 {
            for item in world.load_tile_mut(TilePos::new(1, 0) + dir).items.iter() {
                match item.item_type {
                    ItemType::Corpse(..) => {
                        corpse = Some(item.clone());
                    }
                    ItemType::GraveStone(..) => {
                        gravestone = Some(item.clone());
                    }
                    _ => {}
                }
            }
        }
        assert!(corpse.is_some());
        if let Some(corpse) = corpse {
            if let ItemType::Corpse(ch, body) = corpse.item_type {
                assert_eq!("test", ch.name);
                assert_eq!(SkinTone::Amber, ch.skin_tone);
                assert_eq!(Gender::Male, ch.gender);
                assert_eq!(25, ch.age);
                assert_eq!(MainHand::Right, ch.main_hand);
                assert!(matches!(
                    body.parts
                        .get("torso")
                        .unwrap()
                        .item_type
                        .body_part()
                        .unwrap()
                        .freshness,
                    Freshness::Rotten
                ));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        assert!(gravestone.is_some());
        if let Some(gravestone) = gravestone {
            if let ItemType::GraveStone(data) = gravestone.item_type {
                assert_eq!("test", data.character.name);
                assert_eq!(SkinTone::Amber, data.character.skin_tone);
                assert_eq!(Gender::Male, data.character.gender);
                assert_eq!(25, data.character.age);
                assert_eq!(MainHand::Right, data.character.main_hand);
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }
}
