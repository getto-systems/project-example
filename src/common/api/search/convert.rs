use std::num::TryFromIntError;

use crate::common::api::search::data::{
    SearchCountExtract, SearchSortOrder, SearchSortOrderExtract,
};

impl SearchSortOrderExtract for String {
    fn convert(self) -> Option<SearchSortOrder> {
        for order in SearchSortOrder::variants() {
            if self == order.extract() {
                return Some(order);
            }
        }

        None
    }
}

impl SearchCountExtract<TryFromIntError> for usize {
    fn convert(self) -> Result<i32, TryFromIntError> {
        self.try_into()
    }
}

impl SearchCountExtract<TryFromIntError> for i64 {
    fn convert(self) -> Result<i32, TryFromIntError> {
        self.try_into()
    }
}

#[cfg(test)]
mod test {
    use std::num::TryFromIntError;

    use pretty_assertions::assert_eq;

    use crate::common::api::search::data::{SearchCount, SearchSortOrder};

    #[test]
    fn success_convert_search_sort_order() {
        assert_eq!(
            SearchSortOrder::convert("normal".to_owned()),
            Some(SearchSortOrder::Normal),
        );
    }

    #[test]
    fn success_convert_search_count_usize() -> Result<(), TryFromIntError> {
        let count: usize = 10;
        assert_eq!(SearchCount::convert(count)?.extract(), 10);
        Ok(())
    }

    #[test]
    fn success_convert_search_count_i64() -> Result<(), TryFromIntError> {
        let count: i64 = 10;
        assert_eq!(SearchCount::convert(count)?.extract(), 10);
        Ok(())
    }
}
