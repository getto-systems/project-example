import {
    readSearchOffset,
    updateSearchOffset,
} from "../../../../../common/util/search/offset/convert"
import { readSearchSort, updateSearchSort } from "../../../../../common/util/search/sort/convert"
import {
    readMultipleFilterBoardFilter,
    readSingleFilterBoardValue,
    updateMultipleFilterBoardFilter,
    updateSingleFilterBoardValue,
} from "../../../../../common/util/board/filter/convert"
import { restoreAuthPermission } from "../../../kernel/input/field/convert"

import {
    defaultSearchAuthUserAccountSort,
    SearchAuthUserAccountFilterData,
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountSortKey,
    searchAuthUserAccountSortKeys,
} from "../data"
import { ReadSearchSortKeyResult } from "../../../../../common/util/search/sort/data"
import { DetectFocusListKeyResult } from "../../../../../common/util/list/data"

const FOCUS_ID = "id" as const

export function detectSearchAuthUserAccountFilter(
    currentURL: URL,
): SearchAuthUserAccountFilterData {
    const params = currentURL.searchParams
    return {
        offset: readSearchOffset(params),
        sort: readSearchSort(
            params,
            defaultSearchAuthUserAccountSort,
            readSearchAuthUserAccountSortKey,
        ),
        filter: {
            loginId: readSingleFilterBoardValue(params, filterName("loginId")),
            granted: readMultipleFilterBoardFilter(
                params,
                filterName("granted"),
                restoreAuthPermission,
            ).flat(),
        },
    }
}
export function readSearchAuthUserAccountSortKey(
    key: string,
): ReadSearchSortKeyResult<SearchAuthUserAccountSortKey> {
    for (const sortKey of searchAuthUserAccountSortKeys) {
        if (key === sortKey) {
            return { found: true, key }
        }
    }
    return { found: false }
}

export function detectFocusAuthUserAccount(currentURL: URL): DetectFocusListKeyResult {
    const loginId = currentURL.searchParams.get(FOCUS_ID)
    if (loginId === null) {
        return { found: false }
    }
    return { found: true, key: loginId }
}

export function updateSearchAuthUserAccountFilterQuery(
    currentURL: URL,
    search: SearchAuthUserAccountFilterData,
): URL {
    let url = new URL(currentURL.toString())
    url = updateSearchOffset(url, search.offset)
    url = updateSearchSort(url, search.sort)

    url = updateSingleFilterBoardValue(url, filterName("loginId"), search.filter.loginId)
    url = updateMultipleFilterBoardFilter(
        url,
        filterName("granted"),
        search.filter.granted,
        (filter) => filter,
    )

    return url
}
export function updateFocusAuthUserAccountQuery(currentURL: URL, key: string): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.set(FOCUS_ID, key)
    return url
}
export function clearFocusAuthUserAccountQuery(currentURL: URL): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.delete(FOCUS_ID)
    return url
}

function filterName(key: keyof SearchAuthUserAccountFilter): string {
    return `filter-${key}`
}
