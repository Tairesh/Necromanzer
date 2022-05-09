use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tetra::TetraError;

use settings::time::Time;
use settings::window::Window;

const PATH: &str = "./settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub window: Window,
    #[serde(skip)]
    pub time: Time,
    pub show_fps: bool,
    pub repeat_interval: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: Window::default(),
            time: Time::default(),
            show_fps: false,
            repeat_interval: 75,
        }
    }
}

impl Settings {
    pub fn load() -> tetra::Result<Self> {
        let path = Path::new(PATH);
        let settings = if path.is_file() {
            let file = File::open(path).map_err(|e| TetraError::PlatformError(e.to_string()))?;
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Settings::default())
        } else {
            let settings = Settings::default();
            serde_json::to_writer(
                &File::create(path).map_err(|e| TetraError::PlatformError(e.to_string()))?,
                &settings,
            )
            .map_err(|e| TetraError::PlatformError(e.to_string()))?;
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
