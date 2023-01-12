use crate::common::api::repository::helper::repository_infra_error;

use crate::common::api::{
    repository::data::RepositoryError,
    search::data::{
        SearchCount, SearchLimit, SearchOffset, SearchOffsetLimit, SearchOffsetValue, SearchPage,
        SearchSortDirection, SearchSortOrder,
    },
};

pub fn clip_search<T>(
    mut list: Vec<T>,
    offset: SearchOffsetValue,
    limit: SearchLimit,
) -> Result<(Vec<T>, SearchPage), RepositoryError> {
    let count = SearchCount::convert(list.len())
        .map_err(|err| repository_infra_error("convert offset", err))?;

    let page = SearchOffset::convert(offset, limit, count);

    let SearchOffsetLimit { offset, limit } = page
        .extract_as()
        .map_err(|err| repository_infra_error("convert offset limit", err))?;

    let mut list = list.split_off(offset);
    list.truncate(limit);

    Ok((list, page))
}

pub fn sort_normal(order: SearchSortOrder) -> SearchSortDirection {
    match order {
        SearchSortOrder::Normal => SearchSortDirection::Asc,
        SearchSortOrder::Reverse => SearchSortDirection::Desc,
    }
}

pub fn sort_reverse(order: SearchSortOrder) -> SearchSortDirection {
    match order {
        SearchSortOrder::Normal => SearchSortDirection::Desc,
        SearchSortOrder::Reverse => SearchSortDirection::Asc,
    }
}

pub fn sort_search<T, K: Ord>(
    mut list: Vec<T>,
    key: impl FnMut(&T) -> K,
    order: SearchSortDirection,
) -> Vec<T> {
    list.sort_by_cached_key(key);
    match order {
        SearchSortDirection::Asc => (),
        SearchSortDirection::Desc => list.reverse(),
    }
    list
}
