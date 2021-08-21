pub struct OutlineMenuBadge {
    // badge をつけるターゲット: badge 数
    pub index: OutlineMenuBadgeCount,
}

pub struct OutlineMenuBadgeCount(i32);

impl OutlineMenuBadgeCount {
    pub const fn restore(count: i32) -> Self {
        Self(count)
    }

    pub const fn extract(self) -> i32 {
        self.0
    }
}
