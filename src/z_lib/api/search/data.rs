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

pub struct SearchSort {
    key: String,
    order: SearchSortOrder,
}

impl SearchSort {
    pub fn detect<T, O>(&self, keys: Vec<(&str, T, SearchSortOrderMap<O>)>) -> Option<(T, O)> {
        keys.into_iter().find_map(|(key, col, map)| {
            if key.to_string() == self.key {
                Some((
                    col,
                    match self.order {
                        SearchSortOrder::Normal => map.normal,
                        SearchSortOrder::Reverse => map.reverse,
                    },
                ))
            } else {
                None
            }
        })
    }
}

pub struct SearchSortOrderMap<T> {
    pub normal: T,
    pub reverse: T,
}

pub struct SearchSortExtract {
    pub key: String,
    pub order: String,
}
impl Into<SearchSort> for SearchSortExtract {
    fn into(self) -> SearchSort {
        SearchSort {
            key: self.key,
            order: if self.order == "reverse".to_string() {
                SearchSortOrder::Reverse
            } else {
                SearchSortOrder::Normal
            },
        }
    }
}

pub enum SearchSortOrder {
    Normal,
    Reverse,
}

pub struct SearchPage {
    pub offset: i32,
    pub limit: i32,
    pub all: i32,
}
