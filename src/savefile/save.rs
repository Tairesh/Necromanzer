use savefile::SAVEFILES_FOLDER;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum SaveError {
    SystemError(String),
    SerializeError(String),
    FileExists,
}

impl From<serde_json::Error> for SaveError {
    fn from(e: serde_json::Error) -> Self {
        SaveError::SerializeError(e.to_string())
    }
}

impl From<std::io::Error> for SaveError {
    fn from(e: std::io::Error) -> Self {
        SaveError::SystemError(e.to_string())
    }
}

pub fn create(name: &str, seed: &str) -> Result<(), SaveError> {
    make_dir()?;
    let name = name.trim().replace('\n', "");
    let path = name_to_path(name.as_str());
    if path.is_file() {
        return Err(SaveError::FileExists);
    }
    let mut file = File::create(&path).map_err(SaveError::from)?;
    file.write_all(make_data(name.as_str(), seed)?.as_bytes())
        .map_err(|e| e.into())
}

fn make_dir() -> Result<(), SaveError> {
    let dir = Path::new(SAVEFILES_FOLDER);
    if !dir.exists() {
        std::fs::create_dir(dir).map_err(SaveError::from)?;
    }
    Ok(())
}

fn name_to_path(name: &str) -> PathBuf {
    let file_name = name.replace(' ', "_").replace('/', "").replace('\\', "");
    [SAVEFILES_FOLDER, (file_name + ".save").as_str()]
        .iter()
        .collect()
}

fn make_data(name: &str, seed: &str) -> Result<String, SaveError> {
    // TODO: create initial things and save it through JSON
    Ok(format!("{}\n{}", name, seed))
}
