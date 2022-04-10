use app::App;
use savefile;
use savefile::Meta;
use scenes::implements;
use scenes::scene::Scene;
use tetra::Context;

#[derive(Debug, Clone)]
pub enum GameScene {
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

impl GameScene {
    pub fn into_impl(self, app: &App, ctx: &mut Context) -> Box<dyn Scene> {
        match self {
            GameScene::MainMenu => Box::new(implements::MainMenu::new(app)),
            GameScene::Empty => Box::new(implements::Empty {}),
            GameScene::Settings => Box::new(implements::Settings::new(app, ctx)),
            GameScene::CreateWorld => Box::new(implements::CreateWorld::new(app, ctx)),
            GameScene::LoadWorld => Box::new(implements::LoadWorld::new(app, ctx)),
            GameScene::CreateCharacter(meta) => {
                Box::new(implements::CreateCharacter::new(meta, app, ctx))
            }
            GameScene::Game(meta) => {
                let world = savefile::load_world(&meta.path, app.assets.game_data.clone()).unwrap();
                Box::new(implements::Game::new(world, app, ctx))
            }
            GameScene::GameMenu => Box::new(implements::GameMenu::new(app)),
        }
    }
}
