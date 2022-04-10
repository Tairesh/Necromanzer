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
