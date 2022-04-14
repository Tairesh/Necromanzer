use game::actions::ActionType;
use geometry::direction::Direction;
use scenes::game_modes::GameMode;
use scenes::transition::Transition;

pub enum UpdateResult {
    SceneTransit(Vec<Transition>),
    // ChangeGameView{ zoom_delta: i8 },
    ZoomIn,
    ZoomOut,
    TryRotate(Direction),
    TryStartAction(ActionType),
    ClearLog,
    Examine(Direction),
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
