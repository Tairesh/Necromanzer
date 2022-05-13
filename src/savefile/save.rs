use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use game::World;
use savefile::meta::Meta;
use savefile::SAVEFILES_FOLDER;

#[derive(Debug)]
pub enum Error {
    System(String),
    Serialize(String),
    FileExists,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serialize(e.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::System(e.to_string())
    }
}

pub fn create(name: &str, seed: &str) -> Result<PathBuf, Error> {
    make_dir()?;
    let name = name.trim().replace('\n', "");
    let path = name_to_path(name.as_str());
    if path.is_file() {
        return Err(Error::FileExists);
    }
    let mut file = File::create(&path).map_err(Error::from)?;
    file.write_all(make_data(name.as_str(), seed)?.as_bytes())
        .map_err(Into::into)
        .map(|_| path)
}

pub fn save(world: &mut World) -> Result<(), Error> {
    make_dir()?;
    world.meta.update_before_save();
    let mut file = File::create(&world.meta.path).map_err(Error::from)?;
    file.write_all(make_data_form_world(world)?.as_bytes())
        .map_err(Into::into)
}

fn make_dir() -> Result<(), Error> {
    let dir = Path::new(SAVEFILES_FOLDER);
    if !dir.exists() {
        std::fs::create_dir(dir).map_err(Error::from)?;
    }
    Ok(())
}

fn name_to_path(name: &str) -> PathBuf {
    let file_name = name.replace(' ', "_").replace('/', "").replace('\\', "");
    [SAVEFILES_FOLDER, (file_name + ".save").as_str()]
        .iter()
        .collect()
}

fn make_data(name: &str, seed: &str) -> Result<String, Error> {
    let metadata = Meta::new(name, seed);
    serde_json::to_string(&metadata).map_err(Error::from)
}

fn make_data_form_world(world: &World) -> Result<String, Error> {
    let mut data = serde_json::to_string(&world.meta).map_err(Error::from)?;
    data.push('\n');
    data.push_str(
        serde_json::to_string(&world.game_view)
            .map_err(Error::from)?
            .as_str(),
    );
    for unit in &world.units {
        data.push('\n');
        data.push_str(serde_json::to_string(unit).map_err(Error::from)?.as_str());
    }
    data.push_str("\n/units");
    for coords in world.changed.clone() {
        let chunk = world.get_chunk(coords).unwrap();
        data.push('\n');
        data.push_str(serde_json::to_string(chunk).map_err(Error::from)?.as_str());
    }
    data.push_str("\n/chunks");

    Ok(data)
}
