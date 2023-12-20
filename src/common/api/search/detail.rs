use crate::common::api::{
    repository::data::RepositoryError,
    search::data::{
        Search, SearchCount, SearchLimit, SearchOffset, SearchOffsetLimit, SearchPage, SearchProps,
        SearchSortDirection, SearchSorter,
    },
};

impl<K: Copy, T> Search<K, T> for Vec<T> {
    fn search<D, S>(
        self,
        search: SearchProps<K>,
        limit: SearchLimit,
        matcher: impl FnMut(&T) -> bool,
        key: impl FnOnce(K) -> (D, Box<dyn FnMut(&T) -> S>),
    ) -> Result<(Self, SearchPage), RepositoryError>
    where
        D: SearchSorter,
        S: Ord,
    {
        let mut list: Self = self.into_iter().filter(matcher).collect();

        let (sorter, key) = key(search.sort.key);

        list.sort_by_cached_key(key);

        match sorter.sort(search.sort.order) {
            SearchSortDirection::Asc => (),
            SearchSortDirection::Desc => list.reverse(),
        }

        let count = SearchCount::convert(list.len()).map_err(|err| ("convert offset", err))?;
        let page = SearchOffset::convert(search.offset, limit, count);

        let SearchOffsetLimit { offset, limit } = page
            .extract_as()
            .map_err(|err| ("convert offset limit", err))?;

        let mut list = list.split_off(offset);
        list.truncate(limit);

        Ok((list, page))
    }
}

#[cfg(test)]
pub mod test {
    pub struct MockSearchFilter;

    impl MockSearchFilter {
        pub fn always_returns_true<F, T>(_filter: F) -> impl FnMut(&T) -> bool {
            |_| true
        }
    }
}
