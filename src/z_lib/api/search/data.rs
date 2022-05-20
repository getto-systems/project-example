use std::num::TryFromIntError;

pub struct SearchPage {
    pub offset: i32,
    pub limit: i32,
    pub all: i32,
}

pub struct SearchOffsetDetecter {
    all: i32,
    limit: i32,
}

impl SearchOffsetDetecter {
    pub fn convert<E>(detecter: impl TryInto<Self, Error = E>) -> Result<Self, E> {
        detecter.try_into()
    }

    pub fn detect_page(self, offset: i32) -> SearchPage {
        SearchPage {
            offset: self.detect(offset),
            limit: self.limit,
            all: self.all,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SearchOffsetDetecterExtract<T> {
    pub all: T,
    pub limit: T,
}

impl TryInto<SearchOffsetDetecter> for SearchOffsetDetecterExtract<usize> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<SearchOffsetDetecter, Self::Error> {
        Ok(SearchOffsetDetecter {
            all: self.all.try_into()?,
            limit: self.limit.try_into()?,
        })
    }
}
impl TryInto<SearchOffsetDetecter> for SearchOffsetDetecterExtract<u64> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<SearchOffsetDetecter, Self::Error> {
        Ok(SearchOffsetDetecter {
            all: self.all.try_into()?,
            limit: self.limit.try_into()?,
        })
    }
}

impl SearchOffsetDetecter {
    fn detect(&self, offset: i32) -> i32 {
        if offset < 0 {
            return 0;
        }
        if offset >= self.all {
            return self.last_page();
        }
        self.offset_floor(offset)
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
    pub fn order(&self) -> SearchSortOrder {
        self.order
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

#[derive(Clone, Copy)]
pub enum SearchSortOrder {
    Normal,
    Reverse,
}

pub enum SearchSortOrderExtract {
    Asc,
    Desc,
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
