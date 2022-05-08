#![allow(dead_code)]

use super::implements::*;
use super::{ActionImpl, ActionPossibility};
use enum_dispatch::enum_dispatch;
use game::{Avatar, World};

#[enum_dispatch(ActionImpl)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Copy, Clone)]
pub enum ActionType {
    Skip,
    Walk,
    Wield,
    Drop,
    Dig,
    Read,
    Raise, // TODO: write test for animate
}

#[cfg(test)]
mod tests {
    use super::super::super::human::body::{BodyPartData, Freshness};
    use super::super::super::human::character::Character;
    use super::super::super::human::gender::Gender;
    use super::super::super::human::main_hand::MainHand;
    use super::super::super::human::skin_tone::SkinTone;
    use super::super::super::map::item::Item;
    use super::super::super::map::items::{Axe, BodyPart, Gravestone, Shovel};
    use super::super::super::map::pos::TilePos;
    use super::super::super::map::terrain::Terrain;
    use super::super::super::map::terrains::{Boulder, BoulderSize};
    use super::super::super::map::terrains::{Dirt, Grave, GraveData, GraveVariant};
    use super::super::super::world::tests::add_zombie;
    use super::super::super::world::tests::prepare_world;
    use super::super::{Action, ActionResult};
    use game::actions::implements::*;
    use game::actions::ActionImpl;
    use geometry::direction::{Direction, DIR8};

    #[test]
    fn test_walking() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();

        let typ = Walk {
            dir: Direction::East,
        };
        let length = typ.length(world.player(), &world);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(1, 0), world.player().pos);
    }

    #[test]
    fn test_walking_fail_to_impassable_terrain() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Boulder::new(BoulderSize::Huge).into();

        let typ = Walk {
            dir: Direction::East,
        };
        let length = typ.length(world.player(), &world);
        assert_eq!(0, length);
        assert!(Action::new(0, typ.into(), &world).is_err());
    }

    #[test]
    fn test_walking_fail_to_unit() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();
        add_zombie(&mut world, TilePos::new(1, 0));

        assert!(Action::new(
            0,
            Walk {
                dir: Direction::East
            }
            .into(),
            &world
        )
        .is_err());
    }

    #[test]
    fn test_fail_walking_two_units_to_same_place() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 1)).terrain = Dirt::default().into();
        let zombie = add_zombie(&mut world, TilePos::new(1, 0));

        world.player_mut().action = Some(
            Action::new(
                0,
                Walk {
                    dir: Direction::SouthEast,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.get_unit_mut(zombie).action = Some(
            Action::new(
                zombie,
                Walk {
                    dir: Direction::South,
                }
                .into(),
                &world,
            )
            .unwrap(),
        );
        world.tick();
        assert_eq!(TilePos::new(1, 1), world.player().pos);
        assert_eq!(TilePos::new(1, 0), world.get_unit(zombie).pos);
        assert!(world.player().action.is_none());

        world.player_mut().action = Some(Action::new(0, Skip {}.into(), &world).unwrap());
        world.tick();
        // do not check zombie.action because it can be already new one, selected by AI
        assert_eq!(TilePos::new(1, 0), world.get_unit(zombie).pos);
        assert_eq!(1, world.get_tile(TilePos::new(1, 1)).unwrap().units.len());
        assert_eq!(1, world.get_tile(TilePos::new(1, 0)).unwrap().units.len());
        assert_eq!(0, world.get_tile(TilePos::new(0, 0)).unwrap().units.len());
    }

    #[test]
    fn test_wielding() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(1, 0)).items.clear();
        world
            .load_tile_mut(TilePos::new(1, 0))
            .items
            .push(Axe::new().into());

        assert!(world.player().wield.is_empty());
        assert_eq!(0, world.meta.current_tick);

        let typ = Wield {
            dir: Direction::East,
        };
        let length = typ.length(world.player(), &world);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(1, world.player().wield.len());
        let item = world.player().wield.first().unwrap();
        assert!(matches!(item, Item::Axe(..)));
    }

    #[test]
    fn test_skipping_time() {
        let mut world = prepare_world();

        assert_eq!(0, world.meta.current_tick);
        let typ = Skip {};
        let length = typ.length(world.player(), &world);
        assert_eq!(1, length);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();
        assert_eq!(1, world.meta.current_tick);
    }

    #[test]
    fn test_dropping() {
        let mut world = prepare_world();
        world.load_tile_mut(TilePos::new(0, 0)).terrain = Dirt::default().into();
        world.load_tile_mut(TilePos::new(0, 0)).items.clear();
        world.player_mut().wield.clear();
        world.player_mut().wield.push(Axe::new().into());

        let typ = Drop {
            item_id: 0,
            dir: Direction::Here,
        };
        let length = typ.length(world.player(), &world);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        world.tick();

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert_eq!(0, world.player().wield.len());
        assert_eq!(1, world.load_tile(TilePos::new(0, 0)).items.len());
        let item = world.load_tile(TilePos::new(0, 0)).items.first().unwrap();
        assert!(matches!(item, Item::Axe(..)));
    }

    #[test]
    fn test_digging() {
        let mut world = prepare_world();
        world.player_mut().wield.clear();
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Dirt::default().into();

        let typ = Dig {
            dir: Direction::East,
        };
        let length = typ.length(world.player(), &world);
        assert!(Action::new(0, typ.into(), &world).is_err());

        world.player_mut().wield.push(Shovel::new().into());
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }

        assert_eq!(length as u128, world.meta.current_tick);
        assert_eq!(TilePos::new(0, 0), world.player().pos);
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));

        let character = Character::new("test", Gender::Male, 25, MainHand::Right, SkinTone::Amber);
        world.load_tile_mut(TilePos::new(1, 0)).terrain = Grave::new(
            GraveVariant::New,
            GraveData {
                character,
                death_year: 255,
            },
        )
        .into();
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            world.tick();
        }
        assert!(matches!(
            world.load_tile(TilePos::new(1, 0)).terrain,
            Terrain::Pit(..)
        ));
        let mut corpse = None;
        let mut gravestone = None;
        for dir in DIR8 {
            for item in world.load_tile_mut(TilePos::new(1, 0) + dir).items.iter() {
                match item {
                    Item::Corpse(..) => {
                        corpse = Some(item.clone());
                    }
                    Item::Gravestone(..) => {
                        gravestone = Some(item.clone());
                    }
                    _ => {}
                }
            }
        }
        assert!(corpse.is_some());
        if let Some(corpse) = corpse {
            if let Item::Corpse(corpse) = corpse {
                let ch = &corpse.character;
                let body = &corpse.body;
                assert_eq!("test", ch.name);
                assert_eq!(SkinTone::Amber, ch.skin_tone);
                assert_eq!(Gender::Male, ch.gender);
                assert_eq!(25, ch.age);
                assert_eq!(MainHand::Right, ch.main_hand);
                assert!(matches!(
                    body.parts.get("torso"),
                    Some(Item::BodyPart(BodyPart {
                        data: BodyPartData {
                            freshness: Freshness::Rotten,
                            ..
                        },
                        ..
                    }))
                ));
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
        assert!(gravestone.is_some());
        if let Some(gravestone) = gravestone {
            if let Item::Gravestone(gravestone) = gravestone {
                let data = &gravestone.data;
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

    #[test]
    fn test_reading() {
        let mut world = prepare_world();

        let character = Character::new("test", Gender::Male, 25, MainHand::Right, SkinTone::Amber);
        let data = GraveData {
            character,
            death_year: 255,
        };
        world.load_tile_mut(TilePos::new(1, 0)).terrain =
            Grave::new(GraveVariant::New, data.clone()).into();
        let typ = Read {
            dir: Direction::East,
        };
        let length = typ.length(world.player(), &world);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            let results = world.tick();
            for result in results {
                match result {
                    ActionResult::LogMessage(s) => {
                        assert_eq!("You read on gravestone: test. 230 — 255", s);
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(length as u128, world.meta.current_tick);

        world.load_tile_mut(TilePos::new(0, 1)).terrain = Dirt::default().into();
        world.load_tile_mut(TilePos::new(0, 1)).items.clear();
        let typ = Read {
            dir: Direction::South,
        };
        assert!(Action::new(0, typ.into(), &world).is_err());

        world
            .load_tile_mut(TilePos::new(0, 1))
            .items
            .push(Gravestone::new(data).into());

        let length = typ.length(world.player(), &world);
        world.player_mut().action = Some(Action::new(0, typ.into(), &world).unwrap());
        while world.player().action.is_some() {
            let results = world.tick();
            for result in results {
                match result {
                    ActionResult::LogMessage(s) => {
                        assert_eq!("You read on gravestone: test. 230 — 255", s);
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(length as u128 * 2, world.meta.current_tick);
    }
}
