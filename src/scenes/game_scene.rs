use app::App;
use scenes::implements::main_menu::MainMenu;
use scenes::scene::Scene;
use tetra::Context;

#[derive(Debug, Clone)]
pub enum GameScene {
    MainMenu,
    // #[allow(dead_code)]
    // Empty,
    // Settings,
    // CreateWorld,
    // LoadWorld,
    // CreateCharacter(SaveFile),
    // GameMenu,
}

impl GameScene {
    pub fn to_impl(&self, app: &App, ctx: &mut Context) -> Box<dyn Scene> {
        match self {
            GameScene::MainMenu => Box::new(MainMenu::new(ctx, &app.assets, &app.settings)),
        }
    }
}
