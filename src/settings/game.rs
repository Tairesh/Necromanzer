use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use settings::time::TimeSettings;
use settings::window::WindowSettings;

const PATH: &str = "./settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameSettings {
    pub window_settings: WindowSettings,
    #[serde(skip)]
    pub time_settings: TimeSettings,
    pub show_fps: bool,
    pub repeat_interval: u32,
    pub tile_size: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_settings: WindowSettings::default(),
            time_settings: TimeSettings::default(),
            show_fps: false,
            repeat_interval: 75,
            tile_size: 10.0,
        }
    }
}

impl GameSettings {
    pub fn load() -> tetra::Result<Self> {
        let path = Path::new(PATH);
        let settings = if path.is_file() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| GameSettings::default())
        } else {
            let settings = GameSettings::default();
            serde_json::to_writer(&File::create(path).unwrap(), &settings).unwrap();
            settings
        };
        // TODO: settings.validate();

        Ok(settings)
    }

    pub fn save(&mut self) {
        // TODO: self.validate();
        serde_json::to_writer(&File::create(Path::new(PATH)).unwrap(), self).ok();
    }
}
