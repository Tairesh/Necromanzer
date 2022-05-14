use game::log::category::LogCategory;
use game::map::pos::TilePos;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LogEvent {
    #[serde(rename = "m")]
    pub msg: String,
    #[serde(rename = "p")]
    pub pos: TilePos,
    #[serde(rename = "c")]
    pub category: LogCategory,
}

impl LogEvent {
    pub fn new<S: Into<String>>(msg: S, pos: TilePos, category: LogCategory) -> Self {
        Self {
            msg: msg.into(),
            pos,
            category,
        }
    }
}
