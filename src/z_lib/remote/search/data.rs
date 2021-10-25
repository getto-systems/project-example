pub struct SearchOffsetDetecter {
    pub all: u32,
    pub limit: u32,
}

impl SearchOffsetDetecter {
    pub fn detect(&self, offset: u32) -> u32 {
        if offset >= self.all {
            self.offset_floor(self.all)
        } else {
            self.offset_floor(offset)
        }
    }
    fn offset_floor(&self, offset: u32) -> u32 {
        offset - (offset % self.limit)
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
    pub offset: u32,
    pub limit: u32,
    pub all: u32,
}
