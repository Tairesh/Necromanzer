#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameView {
    pub zoom: i32,
}

impl Default for GameView {
    fn default() -> Self {
        Self { zoom: 2 }
    }
}
