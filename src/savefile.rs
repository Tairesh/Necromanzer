use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct SaveFile {
    path: PathBuf,
    name: String,
    seed: String,
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
        }
    }

    pub fn load(path: PathBuf) -> Option<Self> {
        let file = File::open(&path).ok()?;
        let mut lines = BufReader::new(&file).lines();
        let name = lines.next()?.ok()?;
        let seed = lines.next()?.ok()?;
        Some(SaveFile { path, name, seed })
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
            let data = self.name.clone() + "\n" + self.seed.clone().as_str();
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
