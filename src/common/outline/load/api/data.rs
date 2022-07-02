use std::collections::HashMap;

use crate::x_content::menu::badge::OutlineMenuBadgePath;

pub struct OutlineMenuBadge(HashMap<OutlineMenuBadgePath, OutlineMenuBadgeCount>);

impl OutlineMenuBadge {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self, item: OutlineMenuBadgePath, count: OutlineMenuBadgeCount) {
        self.0.insert(item, count);
    }

    pub fn extract(self) -> HashMap<OutlineMenuBadgePath, OutlineMenuBadgeCount> {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct OutlineMenuBadgeCount(i32);

impl OutlineMenuBadgeCount {
    pub const fn restore(count: i32) -> Self {
        Self(count)
    }

    pub const fn extract(self) -> i32 {
        self.0
    }
}
