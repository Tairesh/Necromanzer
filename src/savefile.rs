use std::fs::{create_dir, remove_file, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use CARGO_VERSION;

pub struct SaveFile {
    pub path: PathBuf,
    pub name: String,
    pub seed: String,
    pub version: String,
    pub time: SystemTime,
}

pub enum SaveFileError {
    SystemError(String),
    FileExists,
}

impl SaveFile {
    pub fn new(name: &str, seed: &str) -> Self {
        let name = name
            .trim()
            .replace("\n", "")
            .replace("/", "")
            .replace("\\", "");
        let file_name = name.replace(" ", "_");
        let path: PathBuf = ["save", (file_name + ".save").as_str()].iter().collect();
        SaveFile {
            path,
            name,
            seed: seed.to_string(),
            version: CARGO_VERSION.to_string(),
            time: SystemTime::now(),
        }
    }

    pub fn load(path: PathBuf) -> Option<Self> {
        let file = File::open(&path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let name = lines.next()?.ok()?;
        if name.is_empty() {
            return None;
        }
        let seed = lines.next()?.ok()?;
        if seed.is_empty() {
            return None;
        }
        let version = lines.next()?.ok()?;
        if version.is_empty() {
            return None;
        }
        let time = lines.next()?.ok()?.parse::<u64>().ok()?;
        let time = SystemTime::UNIX_EPOCH + Duration::new(time, 0);
        Some(SaveFile {
            path,
            name,
            seed,
            version,
            time,
        })
    }

    pub fn save(&self) -> Result<(), SaveFileError> {
        let path = Path::new("save");
        if !path.exists() {
            create_dir(path).map_err(|e| SaveFileError::SystemError(e.to_string()))?;
        }
        if self.path.is_file() {
            Err(SaveFileError::FileExists)
        } else {
            let mut file =
                File::create(&self.path).map_err(|e| SaveFileError::SystemError(e.to_string()))?;
            let data = format!(
                "{}\n{}\n{}\n{}",
                self.name,
                self.seed,
                CARGO_VERSION,
                self.time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .ok()
                    .unwrap()
                    .as_secs()
            );
            file.write_all(data.as_bytes())
                .map_err(|e| SaveFileError::SystemError(e.to_string()))?;
            Ok(())
        }
    }
}

pub fn savefiles() -> Vec<SaveFile> {
    let path = Path::new("save");
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            if let Some(s) = SaveFile::load(p.unwrap().path()) {
                files.push(s);
            }
        }
    }
    files
}

pub fn delete(path: PathBuf) {
    if path.exists() {
        remove_file(path).ok();
    }
}
