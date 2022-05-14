pub use self::category::LogCategory;
pub use self::event::LogEvent;

mod category;
mod event;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Log {
    #[serde(rename = "e")]
    events: Vec<LogEvent>,
    #[serde(rename = "i")]
    pushed: usize,
}

impl Log {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            pushed: 0,
        }
    }

    pub fn push(&mut self, event: LogEvent) {
        self.events.push(event);
    }

    pub fn new_events(&mut self) -> &[LogEvent] {
        let events = &self.events[self.pushed..];
        self.pushed = self.events.len();
        events
    }
}

#[cfg(test)]
mod tests {
    use game::log::LogCategory;
    use game::map::pos::TilePos;

    use super::{Log, LogEvent};

    #[test]
    fn test_log() {
        let mut log = Log::new();
        assert_eq!(0, log.new_events().len());
        log.push(LogEvent::new(
            "Test",
            TilePos::new(0, 0),
            LogCategory::Debug,
        ));
        let events = log.new_events();
        assert_eq!(1, events.len());
        assert_eq!("Test", events[0].msg);
        assert_eq!(0, log.new_events().len());
        log.push(LogEvent::new(
            "Test2",
            TilePos::new(1, 1),
            LogCategory::Danger,
        ));
        let events = log.new_events();
        assert_eq!(1, events.len());
        assert_eq!("Test2", events[0].msg);
        assert_eq!(0, log.new_events().len());
    }
}
