use settings::time::TimeSettings;
use settings::window::WindowSettings;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PATH: &str = "./settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GameSettings {
    pub window_settings: WindowSettings,
    #[serde(skip)]
    pub time_settings: TimeSettings,
    pub show_fps: bool,
    pub move_repeat_interval: u8,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            window_settings: WindowSettings::default(),
            time_settings: TimeSettings::default(),
            show_fps: false,
            move_repeat_interval: 75,
        }
    }
}

impl GameSettings {
    pub fn load() -> tetra::Result<Self> {
        let path = Path::new(PATH);
        let settings = if path.is_file() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
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
