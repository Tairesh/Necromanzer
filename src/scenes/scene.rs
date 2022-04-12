use app::App;
use savefile;
use savefile::Meta;
use scenes::implements;
use scenes::scene_impl::SceneImpl;
use tetra::Context;

#[derive(Debug, Clone)]
pub enum Scene {
    MainMenu,
    #[allow(dead_code)]
    Empty,
    Settings,
    CreateWorld,
    LoadWorld,
    CreateCharacter(Meta),
    Game(Meta),
    GameMenu,
}

impl Scene {
    pub fn into_impl(self, app: &App, ctx: &mut Context) -> Box<dyn SceneImpl> {
        match self {
            Scene::MainMenu => Box::new(implements::MainMenu::new(app)),
            Scene::Empty => Box::new(implements::Empty {}),
            Scene::Settings => Box::new(implements::Settings::new(app, ctx)),
            Scene::CreateWorld => Box::new(implements::CreateWorld::new(app, ctx)),
            Scene::LoadWorld => Box::new(implements::LoadWorld::new(app, ctx)),
            Scene::CreateCharacter(meta) => {
                Box::new(implements::CreateCharacter::new(meta, app, ctx))
            }
            Scene::Game(meta) => {
                let world = savefile::load_world(&meta.path, &app.assets).unwrap();
                Box::new(implements::Game::new(world, app, ctx))
            }
            Scene::GameMenu => Box::new(implements::GameMenu::new(app)),
        }
    }
}
