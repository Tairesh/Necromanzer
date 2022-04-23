use assets::names::Names;

#[derive(Debug)]
pub struct GameData {
    pub names: Names,
}

impl GameData {
    pub fn load() -> tetra::Result<Self> {
        Ok(Self {
            names: Names::load()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::GameData;

    #[test]
    fn data_load() {
        let data = GameData::load().unwrap();
        assert!(data.names.male_names.len() > 0);
        assert!(data.names.female_names.len() > 0);
        assert!(data.names.names.len() > 0);
    }
}
