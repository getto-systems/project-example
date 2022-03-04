use std::convert::TryInto;

pub struct SearchOffset {
    pub all: i32,
    pub limit: i32,
}

impl SearchOffset {
    pub fn detect(&self, offset: i32) -> i32 {
        let offset = self.cast_offset(offset);
        if offset >= self.all {
            self.last_page()
        } else {
            self.offset_floor(offset)
        }
    }
    fn cast_offset(&self, offset: i32) -> i32 {
        if offset < 0 {
            0
        } else {
            // マイナスでなければエラーにはならない
            offset.try_into().unwrap()
        }
    }
    fn offset_floor(&self, offset: i32) -> i32 {
        offset - (offset % self.limit)
    }
    fn last_page(&self) -> i32 {
        if self.all % self.limit > 0 {
            return self.offset_floor(self.all);
        }
        if self.all == 0 {
            return 0;
        }
        self.all - self.limit
    }
}

pub struct SearchSort<K> {
    key: K,
    order: SearchSortOrder,
}

impl<K> SearchSort<K> {
    pub fn key(&self) -> &K {
        &self.key
    }
    pub fn order(&self) -> &SearchSortOrder {
        &self.order
    }
}

impl<K: Into<String>> SearchSort<K> {
    pub fn extract(self) -> SearchSortExtract {
        SearchSortExtract {
            key: self.key.into(),
            order: self.order.into(),
        }
    }
}

pub enum SearchSortOrder {
    Normal,
    Reverse,
}

impl Into<String> for SearchSortOrder {
    fn into(self) -> String {
        match self {
            Self::Normal => "normal".to_owned(),
            Self::Reverse => "reverse".to_owned(),
        }
    }
}

pub struct SearchSortExtract {
    pub key: String,
    pub order: String,
}
impl<K: From<String>> Into<SearchSort<K>> for SearchSortExtract {
    fn into(self) -> SearchSort<K> {
        SearchSort {
            key: self.key.into(),
            order: if self.order.as_str() == "reverse" {
                SearchSortOrder::Reverse
            } else {
                SearchSortOrder::Normal
            },
        }
    }
}

pub struct SearchPage {
    pub offset: i32,
    pub limit: i32,
    pub all: i32,
}
