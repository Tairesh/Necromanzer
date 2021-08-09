use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PATH: &str = "settings.json";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub borderless: bool,
    pub show_fps: bool,
    pub music_enabled: bool,
    pub music_volume: u8,
}

impl Settings {
    pub fn default() -> Result<Settings, String> {
        Ok(Settings {
            width: 1024,
            height: 768,
            fullscreen: false,
            borderless: false,
            show_fps: false,
            music_enabled: true,
            music_volume: 64,
        })
    }

    pub fn load() -> Result<Settings, String> {
        let path = Path::new(PATH);
        let mut settings: Settings;
        if path.is_file() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            settings = serde_json::from_reader(reader).unwrap();
            settings.validate();
        } else {
            settings = Settings::default()?;
            serde_json::to_writer(&File::create(path).unwrap(), &settings).unwrap();
        }

        Ok(settings)
    }

    pub fn save(&mut self) {
        self.validate();
        serde_json::to_writer(&File::create(Path::new(PATH)).unwrap(), self).unwrap();
    }

    fn validate(&mut self) -> &Settings {
        if self.width < 640 {
            self.width = 640;
        }
        if self.width > 1920 {
            self.width = 1920;
        }
        if self.height < 480 {
            self.height = 480;
        }
        if self.height > 1280 {
            self.height = 1280;
        }
        if self.music_volume > 128 {
            self.music_volume = 128;
        }
        self
    }
}
