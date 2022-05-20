use crate::z_lib::repository::helper::repository_infra_error;

use crate::z_lib::{
    repository::data::RepositoryError,
    search::data::{
        SearchOffsetDetecter, SearchOffsetDetecterExtract, SearchPage, SearchSortOrder,
        SearchSortOrderExtract,
    },
};

pub struct SearchClip {
    pub limit: usize,
    pub offset: i32,
}
pub fn clip_search<T>(
    mut list: Vec<T>,
    clip: SearchClip,
) -> Result<(Vec<T>, SearchPage), RepositoryError> {
    let detecter = SearchOffsetDetecterExtract {
        all: list.len(),
        limit: clip.limit,
    };
    let page = SearchOffsetDetecter::convert(detecter)
        .map_err(|err| repository_infra_error("convert offset error", err))?
        .detect_page(clip.offset);

    let mut list = list.split_off(
        page.offset
            .try_into()
            .map_err(|err| repository_infra_error("convert offset error", err))?,
    );
    list.truncate(detecter.limit);

    Ok((list, page))
}

pub fn sort_normal(order: SearchSortOrder) -> SearchSortOrderExtract {
    match order {
        SearchSortOrder::Normal => SearchSortOrderExtract::Asc,
        SearchSortOrder::Reverse => SearchSortOrderExtract::Desc,
    }
}

pub fn sort_reverse(order: SearchSortOrder) -> SearchSortOrderExtract {
    match order {
        SearchSortOrder::Normal => SearchSortOrderExtract::Desc,
        SearchSortOrder::Reverse => SearchSortOrderExtract::Asc,
    }
}

pub fn sort_search<T, K: Ord>(
    mut list: Vec<T>,
    key: impl FnMut(&T) -> K,
    order: SearchSortOrderExtract,
) -> Vec<T> {
    list.sort_by_cached_key(key);
    match order {
        SearchSortOrderExtract::Asc => (),
        SearchSortOrderExtract::Desc => list.reverse(),
    }
    list
}
