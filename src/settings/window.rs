#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct WindowSettings {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fullscreen: false,
        }
    }
}
