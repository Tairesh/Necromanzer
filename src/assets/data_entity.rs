use map::Passage;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<ItemTag>,
    pub mass: u32,
    pub volume: u32,
    #[serde(default)]
    pub read_action: Option<ReadAction>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReadAction {
    GraveStone,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemTag {
    BodyPart,
    GraveData,
    Corpse,
    Wear,
    Dig,
    Cut,
}

#[derive(Deserialize, Debug)]
pub struct Terrain {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub pass: Passage,
    #[serde(default)]
    pub variants: usize,
    #[serde(default)]
    pub dig_result: Option<String>,
    #[serde(default)]
    pub dig_action: Option<DigAction>,
    #[serde(default)]
    pub read_action: Option<ReadAction>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DigAction {
    Grave,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum DataEntity {
    Item(Item),
    Terrain(Terrain),
}

#[cfg(test)]
mod tests {
    use super::DataEntity;
    use super::DigAction;
    use super::ItemTag;
    use assets::data_entity::ReadAction;
    use map::Passage;

    #[test]
    fn test_deserialize() {
        let json = r#"
        [
          {
            "type": "item",
            "id": "heart",
            "name": "Human heart",
            "mass": 10,
            "volume": 10,
            "tags": [ "BODY_PART" ]
          },
          {
            "type": "item",
            "id": "gravestone",
            "name": "Gravestone",
            "mass": 1000,
            "volume": 100,
            "read_action": "GRAVE_STONE"
          },
          {
            "type": "terrain",
            "id": "dirt",
            "name": "Dirt",
            "variants": 3,
            "pass": { "passable": 10.0 },
            "dig_result": "pit"
          },
          {
            "type": "terrain",
            "id": "grave",
            "name": "Grave",
            "variants": 0,
            "pass": "unpassable",
            "dig_result": "pit",
            "dig_action": "GRAVE",
            "read_action": "GRAVE_STONE"
          }
        ]
        "#;
        let data: Vec<DataEntity> = serde_json::from_str(json).unwrap();
        let slice = data.as_slice();
        assert!(matches!(slice[0], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[0] {
            assert_eq!("heart", item.id);
            assert_eq!("Human heart", item.name);
            assert!(item.description.is_empty());
            assert_eq!(10, item.mass);
            assert_eq!(10, item.volume);
            assert!(item.tags.contains(&ItemTag::BodyPart));
            assert!(item.read_action.is_none());
        } else {
            unreachable!("DataEntity is not Item!");
        }
        assert!(matches!(slice[1], DataEntity::Item(..)));
        if let DataEntity::Item(item) = &slice[1] {
            assert_eq!("gravestone", item.id);
            assert_eq!("Gravestone", item.name);
            assert!(item.description.is_empty());
            assert_eq!(1000, item.mass);
            assert_eq!(100, item.volume);
            assert!(item.tags.is_empty());
            assert!(item.read_action.is_some());
            assert!(matches!(item.read_action, Some(ReadAction::GraveStone)));
        } else {
            unreachable!("DataEntity is not Item!");
        }
        assert!(matches!(slice[2], DataEntity::Terrain(..)));
        if let DataEntity::Terrain(terrain) = &slice[2] {
            assert_eq!("dirt", terrain.id);
            assert_eq!("Dirt", terrain.name);
            assert_eq!(3, terrain.variants);
            assert!(terrain.dig_result.is_some());
            assert_eq!("pit", terrain.dig_result.as_ref().unwrap());
            assert!(matches!(terrain.pass, Passage::Passable(_)));
            assert!(terrain.dig_action.is_none());
            assert!(terrain.read_action.is_none());
        } else {
            unreachable!("DataEntity is not Terrain!");
        }
        assert!(matches!(slice[3], DataEntity::Terrain(..)));
        if let DataEntity::Terrain(terrain) = &slice[3] {
            assert_eq!("grave", terrain.id);
            assert_eq!("Grave", terrain.name);
            assert_eq!(0, terrain.variants);
            assert!(terrain.dig_result.is_some());
            assert_eq!("pit", terrain.dig_result.as_ref().unwrap());
            assert!(matches!(terrain.pass, Passage::Unpassable));
            assert!(terrain.dig_action.is_some());
            assert!(matches!(terrain.dig_action, Some(DigAction::Grave)));
            assert!(terrain.read_action.is_some());
            assert!(matches!(terrain.read_action, Some(ReadAction::GraveStone)));
        } else {
            unreachable!("DataEntity is not Terrain!");
        }
    }
}
