use app::App;
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
    // CreateCharacter(SaveFile),
    // GameMenu,
}

impl GameScene {
    pub fn to_impl(&self, app: &App, ctx: &mut Context) -> Box<dyn Scene> {
        match self {
            GameScene::MainMenu => Box::new(implements::MainMenu::new(app)),
            GameScene::Empty => Box::new(implements::Empty {}),
            GameScene::Settings => Box::new(implements::Settings::new(app, ctx)),
            GameScene::CreateWorld => Box::new(implements::CreateWorld::new(app, ctx)),
            GameScene::LoadWorld => Box::new(implements::LoadWorld::new(app, ctx)),
        }
    }
}
