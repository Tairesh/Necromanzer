use scenes::game_modes::GameMode;
use scenes::transition::Transition;

// TODO: as game modes now in Rc<RefCell<>> it probably doesn't needed at all
pub enum UpdateResult {
    SceneTransit(Vec<Transition>),
    Push(GameMode),
    Replace(GameMode),
    Pop,
}

pub type SomeResults = Option<Vec<UpdateResult>>;

impl From<UpdateResult> for SomeResults {
    fn from(r: UpdateResult) -> Self {
        Some(vec![r])
    }
}
