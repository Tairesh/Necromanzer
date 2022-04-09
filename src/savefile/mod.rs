mod save;

pub use savefile::save::create;
pub use savefile::save::SaveError;

const SAVEFILES_FOLDER: &str = "save";
