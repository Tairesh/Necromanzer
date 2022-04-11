use super::Meta;
use super::SAVEFILES_FOLDER;
use assets::game_data::GameData;
use game::{Avatar, World};
use map::chunk::Chunk;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

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

pub fn have_avatar(path: &Path) -> bool {
    if let Ok(file) = File::open(path) {
        let mut lines = BufReader::new(&file).lines();
        lines.next();
        lines.next();
        lines.next().is_some()
    } else {
        false
    }
}

pub fn load_world(path: &Path, game_data: Rc<GameData>) -> Result<World, LoadError> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(&file).lines();
    let meta = lines.next().unwrap()?;
    let game_view = lines.next().unwrap()?;
    let mut units_data = Vec::new();
    loop {
        let unit = lines.next().unwrap()?;
        if unit.eq("/units") {
            break;
        }
        units_data.push(unit);
    }
    let mut chunks_data = Vec::new();
    loop {
        let chunk = lines.next().unwrap()?;
        if chunk.eq("/chunks") {
            break;
        }
        chunks_data.push(chunk);
    }

    let mut units = Vec::with_capacity(units_data.len());
    for unit in units_data.iter() {
        let unit: Avatar = serde_json::from_str(unit).unwrap();
        units.push(unit);
    }

    let mut chunks = HashMap::with_capacity(chunks_data.len());
    for chunk in chunks_data.iter() {
        let chunk: Chunk = serde_json::from_str(chunk).unwrap();
        chunks.insert(chunk.pos, chunk);
    }
    Ok(World::new(
        serde_json::from_str(meta.as_str()).map(|s: Meta| s.with_path(path))?,
        serde_json::from_str(game_view.as_str())?,
        units,
        chunks,
        game_data,
    ))
}