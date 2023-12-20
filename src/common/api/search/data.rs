use std::num::TryFromIntError;

use crate::common::api::repository::data::RepositoryError;

pub trait Search<K: Copy, T>: Sized {
    fn search<D, S>(
        self,
        filter: SearchProps<K>,
        limit: SearchLimit,
        matcher: impl FnMut(&T) -> bool,
        key: impl FnOnce(K) -> (D, Box<dyn FnMut(&T) -> S>),
    ) -> Result<(Self, SearchPage), RepositoryError>
    where
        D: SearchSorter,
        S: Ord;
}

#[derive(Clone, Copy)]
pub struct SearchProps<K: Copy> {
    pub offset: SearchOffsetValue,
    pub sort: SearchSort<K>,
}

pub trait SearchSorter {
    fn sort(&self, order: SearchSortOrder) -> SearchSortDirection;
}

pub struct SearchSorterNormal;

impl SearchSorter for SearchSorterNormal {
    fn sort(&self, order: SearchSortOrder) -> SearchSortDirection {
        match order {
            SearchSortOrder::Normal => SearchSortDirection::Asc,
            SearchSortOrder::Reverse => SearchSortDirection::Desc,
        }
    }
}

pub struct SearchSorterReverse;

impl SearchSorter for SearchSorterReverse {
    fn sort(&self, order: SearchSortOrder) -> SearchSortDirection {
        match order {
            SearchSortOrder::Normal => SearchSortDirection::Desc,
            SearchSortOrder::Reverse => SearchSortDirection::Asc,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SearchPage {
    pub offset: SearchOffset,
    pub limit: SearchLimit,
    pub count: SearchCount,
}

pub struct SearchOffsetLimit<T> {
    pub offset: T,
    pub limit: T,
}

impl SearchPage {
    pub const fn zero(limit: SearchLimit) -> Self {
        Self {
            offset: SearchOffset(0),
            limit,
            count: SearchCount(0),
        }
    }

    pub fn extract_as<T: TryFrom<i32, Error = TryFromIntError>>(
        self,
    ) -> Result<SearchOffsetLimit<T>, TryFromIntError> {
        Ok(SearchOffsetLimit {
            offset: self.offset.extract().try_into()?,
            limit: self.limit.extract().try_into()?,
        })
    }
}

#[derive(Clone, Copy)]
pub struct SearchOffsetValue(i32);

impl From<i32> for SearchOffsetValue {
    fn from(value: i32) -> Self {
        if value < 0 {
            Default::default()
        } else {
            Self(value)
        }
    }
}

impl Default for SearchOffsetValue {
    fn default() -> Self {
        Self(0)
    }
}

pub enum ValidateSearchOffsetValueError {
    Negative,
}

#[derive(Clone, Copy)]
pub struct SearchOffset(i32);

impl SearchOffset {
    pub const fn convert(
        value: SearchOffsetValue,
        limit: SearchLimit,
        count: SearchCount,
    ) -> SearchPage {
        SearchPage {
            offset: if value.0 < count.0 {
                Self::floor(value.0, limit)
            } else {
                count.into_last_page_offset(limit)
            },
            limit,
            count,
        }
    }
    const fn floor(value: i32, limit: SearchLimit) -> Self {
        Self(if value < 0 {
            0
        } else {
            value - (value % limit.0)
        })
    }

    pub const fn extract(self) -> i32 {
        self.0
    }
}

impl std::fmt::Display for SearchOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "search-offset: {}", self.0)
    }
}

#[derive(Clone, Copy)]
pub struct SearchLimit(i32);

impl SearchLimit {
    pub const fn convert(value: i32) -> Result<Self, ValidateSearchLimitError> {
        if value < 0 {
            Err(ValidateSearchLimitError::Negative)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn extract(self) -> i32 {
        self.0
    }
}

pub enum ValidateSearchLimitError {
    Negative,
}

impl std::fmt::Display for ValidateSearchLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Negative => write!(f, "negative"),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct SearchCount(i32);

impl SearchCount {
    pub fn convert<E>(value: impl SearchCountExtract<E>) -> Result<Self, E> {
        Ok(Self(value.convert()?))
    }

    pub const fn extract(self) -> i32 {
        self.0
    }

    const fn into_last_page_offset(self, limit: SearchLimit) -> SearchOffset {
        if self.0 % limit.0 > 0 {
            SearchOffset::floor(self.0, limit)
        } else {
            SearchOffset::floor(self.0 - limit.0, limit)
        }
    }
}

pub trait SearchCountExtract<E> {
    fn convert(self) -> Result<i32, E>;
}

#[derive(Clone, Copy)]
pub struct SearchSort<K: Copy> {
    pub key: K,
    pub order: SearchSortOrder,
}

impl<K: Default + Copy> Default for SearchSort<K> {
    fn default() -> Self {
        Self {
            key: Default::default(),
            order: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchSortOrder {
    // 変更したら variants() も更新する
    Normal,
    Reverse,
}

impl SearchSortOrder {
    pub const fn variants() -> [Self; 2] {
        [
            // variants が増えたらここにも追加する
            Self::Normal,
            Self::Reverse,
        ]
    }

    pub fn convert(value: impl SearchSortOrderExtract) -> Option<Self> {
        value.convert()
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Reverse => "reverse",
        }
    }
    pub fn extract(self) -> String {
        self.as_str().to_owned()
    }
}

pub trait SearchSortOrderExtract {
    fn convert(self) -> Option<SearchSortOrder>;
}

impl Default for SearchSortOrder {
    fn default() -> Self {
        Self::Normal
    }
}

pub enum SearchSortDirection {
    Asc,
    Desc,
}
