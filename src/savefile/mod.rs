use std::path::Path;

pub use savefile::game_view::GameView;
pub use savefile::load::have_avatar;
pub use savefile::load::load;
pub use savefile::load::load_world;
pub use savefile::load::savefiles;
pub use savefile::load::savefiles_exists;
pub use savefile::meta::Meta;
pub use savefile::save::create;
pub use savefile::save::save;
pub use savefile::save::Error;

mod game_view;
mod load;
mod meta;
mod save;

const SAVEFILES_FOLDER: &str = "save";

pub fn delete(path: &Path) {
    if path.exists() {
        std::fs::remove_file(path).ok();
    }
}

#[cfg(test)]
mod tests {
    use game::world::tests::prepare_world;
    use savefile::{delete, load, load_world};
    use std::path::PathBuf;

    #[test]
    fn test_save_and_load() {
        let path = PathBuf::from("save/test.save");
        let mut world = prepare_world();
        world.meta.path = path.clone();
        world.save();

        let meta = load(&path).unwrap();
        assert_eq!(meta.name, world.meta.name);
        assert_eq!(meta.current_tick, world.meta.current_tick);
        assert_eq!(meta.version, world.meta.version);
        assert_eq!(meta.seed, world.meta.seed);

        let world2 = load_world(&path).unwrap();
        assert_eq!(world.game_view.zoom, world2.game_view.zoom);
        assert_eq!(world.player().pos, world2.player().pos);
        assert_eq!(
            world.player().person().mind.name,
            world2.player().person().mind.name
        );

        delete(&path);
    }
}
