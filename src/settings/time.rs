use time::UtcOffset;

#[derive(Debug)]
pub struct Time {
    pub offset: UtcOffset,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            offset: UtcOffset::current_local_offset().unwrap(),
        }
    }
}
