use std::cell::RefCell;
use std::rc::Rc;

use ui::traits::UiSprite;

pub use self::game_log::GameLog;
pub use self::implements::alert;
pub use self::implements::bg;
pub use self::implements::button;
pub use self::implements::image;
pub use self::implements::inputs;
pub use self::implements::label;
pub use self::implements::meshy;

mod game_log;
mod implements;
pub mod position;
pub mod traits;

pub type BunchOfSprites = Vec<Rc<RefCell<dyn UiSprite>>>;
pub type SomeSprites<'a> = Option<&'a BunchOfSprites>;
