use std::fs::{create_dir, File};
use std::io::Write;
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
