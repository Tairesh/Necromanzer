use savefile::meta::Meta;
use savefile::SAVEFILES_FOLDER;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn savefiles_exists() -> bool {
    let path = Path::new(SAVEFILES_FOLDER);
    path.read_dir()
        .map(|mut read_dir| {
            read_dir.any(|entry| {
                entry
                    .map(|entry| {
                        entry.file_type().map(|t| t.is_file()).unwrap_or(false)
                            && entry
                                .path()
                                .extension()
                                .map(|ext| ext == "save")
                                .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

pub fn savefiles() -> Vec<Meta> {
    let path = Path::new(SAVEFILES_FOLDER);
    let mut files = Vec::new();
    if path.exists() {
        for p in path.read_dir().unwrap() {
            let p = p.unwrap().path();
            if let Some(s) = load(&p) {
                // TODO: some implementation for invalid (old/broken) savefiles
                files.push(s);
            }
        }
    }
    files.sort();
    files.reverse();
    files
}

#[derive(Debug)]
pub enum LoadError {
    SystemError(String),
    DeserializeError(String),
}

impl From<serde_json::Error> for LoadError {
    fn from(e: serde_json::Error) -> Self {
        LoadError::DeserializeError(e.to_string())
    }
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::SystemError(e.to_string())
    }
}

pub fn load(path: &Path) -> Option<Meta> {
    let file = File::open(path).ok()?;
    let mut lines = BufReader::new(&file).lines();
    let meta = lines.next()?.ok()?;
    serde_json::from_str(meta.as_str())
        .ok()
        .map(|s: Meta| s.with_path(path))
}
