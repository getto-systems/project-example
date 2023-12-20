use crate::common::api::search::y_protobuf::service::SearchSortPb;

use crate::common::api::search::data::{SearchSort, SearchSortOrder};

impl<K: Copy + Default + From<String>> From<Option<SearchSortPb>> for SearchSort<K> {
    fn from(data: Option<SearchSortPb>) -> Self {
        match data {
            Some(data) => SearchSort {
                key: data.key.into(),
                order: SearchSortOrder::convert(data.order).unwrap_or_else(|| Default::default()),
            },
            None => Default::default(),
        }
    }
}
