use app::App;
use scenes::implements;
use scenes::scene_impl::SceneImpl;
use std::path::PathBuf;
use tetra::Context;

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    #[allow(dead_code)]
    Empty,
    Settings,
    CreateWorld,
    LoadWorld,
    CreateCharacter(PathBuf),
    Game,
    GameMenu,
}

impl Scene {
    // TODO: add Result<> to all Scene::new()
    pub fn into_impl(self, app: &App, ctx: &mut Context) -> Box<dyn SceneImpl> {
        match self {
            Scene::MainMenu => Box::new(implements::MainMenu::new(app)),
            Scene::Empty => Box::new(implements::Empty {}),
            Scene::Settings => Box::new(implements::Settings::new(app, ctx)),
            Scene::CreateWorld => Box::new(implements::CreateWorld::new(app, ctx)),
            Scene::LoadWorld => Box::new(implements::LoadWorld::new(app, ctx)),
            Scene::CreateCharacter(path) => {
                Box::new(implements::CreateCharacter::new(path, app, ctx))
            }
            Scene::Game => Box::new(implements::Game::new(app, ctx)),
            Scene::GameMenu => Box::new(implements::GameMenu::new(app)),
        }
    }
}
