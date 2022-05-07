use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;

pub mod alert;
pub mod bg;
pub mod button;
pub mod image;
pub mod input;
pub mod label;
pub mod meshy;
pub mod position;
pub mod sprite;

pub type BunchOfSprites = Vec<Rc<RefCell<dyn Sprite>>>;
pub type SomeSprites<'a> = Option<&'a BunchOfSprites>;

// TODO: rename sprites to UI
