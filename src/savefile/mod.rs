mod load;
mod meta;
mod save;

pub use savefile::load::savefiles;
pub use savefile::load::savefiles_exists;
pub use savefile::save::create;
pub use savefile::save::SaveError;
use std::path::Path;

const SAVEFILES_FOLDER: &str = "save";

pub fn delete(path: &Path) {
    if path.exists() {
        std::fs::remove_file(path).ok();
    }
}
