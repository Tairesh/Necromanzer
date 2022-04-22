use assets::data_entity::{DataEntity, Item, Terrain};
use assets::names::Names;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug)]
pub struct GameData {
    pub names: Names,
    pub terrains: GameDataMap<Terrain>,
    pub items: GameDataMap<Item>,
}

#[derive(Debug)]
pub struct GameDataMap<Value> {
    map: HashMap<String, Rc<Value>>,
}

impl<Value> GameDataMap<Value> {
    pub fn new() -> Self {
        Self {
            map: HashMap::with_capacity(10),
        }
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn set<S: Into<String>>(&mut self, id: S, value: Value) {
        self.map.insert(id.into(), Rc::new(value));
    }

    pub fn get(&self, id: &str) -> Rc<Value> {
        self.map.get(id).unwrap().clone()
    }
}

impl<Value> Default for GameDataMap<Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl GameData {
    pub fn load() -> tetra::Result<Self> {
        let mut data = Self {
            names: Names::load()?,
            terrains: GameDataMap::new(),
            items: GameDataMap::new(),
        };
        // TODO: mods support
        let path: PathBuf = ["data", "core"].iter().collect();
        data.load_dir(&path);
        Ok(data)
    }

    fn load_dir(&mut self, path: &Path) {
        for entry in path.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                self.load_dir(&path);
            } else if let Ok(file) = File::open(path) {
                if let Ok(entities) =
                    serde_json::from_reader::<_, Vec<DataEntity>>(BufReader::new(file))
                {
                    for entity in entities {
                        self.add_entity(entity);
                    }
                }
            }
        }
    }

    fn add_entity(&mut self, entity: DataEntity) {
        match entity {
            DataEntity::Item(item) => {
                self.items.set(item.id.clone(), item);
            }
            DataEntity::Terrain(terrain) => {
                self.terrains.set(terrain.id.clone(), terrain);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameData;
    use assets::data_entity::{ItemTag, ReadAction};

    #[test]
    fn data_load() {
        let data = GameData::load().unwrap();
        assert!(data.names.male_names.len() > 0);
        assert!(data.names.female_names.len() > 0);
        assert!(data.names.names.len() > 0);
        assert!(data.terrains.len() > 0);
        assert!(matches!(
            data.terrains.get("grave").read_action,
            Some(ReadAction::GraveStone)
        ));
        assert!(data.items.len() > 0);
        assert!(data
            .items
            .get("human_heart")
            .tags
            .contains(&ItemTag::BodyPart));
    }
}
