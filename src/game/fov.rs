use geometry::point::Point;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Fov {
    set: HashSet<Point>,
    is_dirty: bool,
}

impl Fov {
    pub fn new() -> Self {
        Self {
            set: HashSet::default(),
            is_dirty: false,
        }
    }

    pub fn set_set(&mut self, set: HashSet<Point>) {
        self.set = set;
        self.is_dirty = true;
    }

    pub fn updated_set(&mut self) -> Option<HashSet<Point>> {
        if self.is_dirty {
            self.is_dirty = false;
            Some(self.set.clone())
        } else {
            None
        }
    }

    pub fn last_set(&self) -> HashSet<Point> {
        self.set.clone()
    }
}

impl Default for Fov {
    fn default() -> Self {
        Self::new()
    }
}
