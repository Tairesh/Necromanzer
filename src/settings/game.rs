use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;

use settings::time::Time;
use settings::window::Window;

const DEFAULT_PATH: &str = "./settings.json";

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

static INSTANCE: OnceCell<Mutex<Settings>> = OnceCell::new();

impl Settings {
    pub fn instance() -> MutexGuard<'static, Settings> {
        INSTANCE
            .get_or_init(|| Mutex::new(load(DEFAULT_PATH)))
            .lock()
            .expect("Can't lock Mutex<Settings>!")
    }

    pub fn save(&mut self) {
        // TODO: self.validate();
        save(self, DEFAULT_PATH);
    }
}

fn load_from_file(path: &'static str) -> Result<Settings, ()> {
    let path = Path::new(path);
    if !path.is_file() {
        return Err(());
    }
    let file = File::open(path).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|_| ())
}

fn load(path: &'static str) -> Settings {
    load_from_file(path).unwrap_or_else(|_| {
        let settings = Settings::default();
        save(&settings, path);
        settings
    })
}

fn save(settings: &Settings, path: &'static str) {
    serde_json::to_writer(&File::create(Path::new(path)).unwrap(), settings).ok();
}

#[cfg(test)]
mod tests {
    use settings::game::save;

    use super::load;

    const TEST_PATH: &str = "./settings-test.json";

    #[test]
    fn test_settings_load_and_save() {
        let mut settings = load(TEST_PATH);
        settings.window.width = 987;
        save(&settings, TEST_PATH);

        let settings = load(TEST_PATH);
        assert_eq!(987, settings.window.width);

        std::fs::remove_file(TEST_PATH).ok();
    }
}
