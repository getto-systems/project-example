use crate::common::api::search::data::{
    SearchSortDirection, SearchSortOrder, SearchSorter, SearchSorterNormal, SearchSorterReverse,
};

pub fn sort_normal(order: SearchSortOrder) -> SearchSortDirection {
    SearchSorterNormal.sort(order)
}

pub fn sort_reverse(order: SearchSortOrder) -> SearchSortDirection {
    SearchSorterReverse.sort(order)
}
