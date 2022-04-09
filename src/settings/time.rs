use time::UtcOffset;

#[derive(Debug)]
pub struct TimeSettings {
    pub offset: UtcOffset,
}

impl Default for TimeSettings {
    fn default() -> Self {
        Self {
            offset: UtcOffset::current_local_offset().unwrap(),
        }
    }
}
