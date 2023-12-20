use crate::common::api::search::y_protobuf::service::{SearchPagePb, SearchSortPb};

use crate::common::api::search::data::{SearchPage, SearchSort};

impl Into<SearchPagePb> for SearchPage {
    fn into(self) -> SearchPagePb {
        SearchPagePb {
            offset: self.offset.extract(),
            limit: self.limit.extract(),
            count: self.count.extract(),
        }
    }
}

impl<K: Copy + Into<String>> Into<SearchSortPb> for SearchSort<K> {
    fn into(self) -> SearchSortPb {
        SearchSortPb {
            key: self.key.into(),
            order: self.order.extract(),
        }
    }
}
