pub use self::implements::alert;
pub use self::implements::bg;
pub use self::implements::button;
pub use self::implements::image;
pub use self::implements::input;
pub use self::implements::label;
pub use self::implements::meshy;
use std::cell::RefCell;
use std::rc::Rc;
use ui::traits::UiSprite;

mod implements;
pub mod position;
pub mod traits;

pub type BunchOfSprites = Vec<Rc<RefCell<dyn UiSprite>>>;
pub type SomeSprites<'a> = Option<&'a BunchOfSprites>;
