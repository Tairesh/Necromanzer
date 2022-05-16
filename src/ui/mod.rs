use ui::traits::UiSprite;

pub use self::game_log::GameLog;
pub use self::implements::alert;
pub use self::implements::button;
pub use self::implements::image;
pub use self::implements::inputs;
pub use self::implements::label;
pub use self::implements::meshy;

mod game_log;
mod implements;
pub mod position;
pub mod traits;

pub type SomeUISpritesMut<'a> = Option<&'a mut [Box<dyn UiSprite>]>;
pub type SomeUISprites<'a> = Option<&'a [Box<dyn UiSprite>]>;
