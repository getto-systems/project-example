import { readSearchOffset, updateSearchOffset } from "../../../../common/util/search/offset/convert"
import { readSearchSort, updateSearchSort } from "../../../../common/util/search/sort/convert"
import {
    readSearchMultipleValueFilter,
    readSearchTextFilter,
    updateSearchMultipleValueFilter,
    updateSearchTextFilter,
} from "../../../../common/util/search/kernel/convert"

import {
    defaultSearchAuthUserAccountSort,
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountFilterProps,
    SearchAuthUserAccountSortKey,
    searchAuthUserAccountSortKeys,
} from "./data"
import { ReadSearchSortKeyResult } from "../../../../common/util/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { toGranted } from "../input/granted/convert"
import { DetectFocusListKeyResult } from "../../../../common/util/list/data"

const FOCUS_ID = "id" as const

export function detectSearchAuthUserAccountFilter(currentURL: URL): SearchAuthUserAccountFilter {
    const params = currentURL.searchParams
    return {
        offset: readSearchOffset(params),
        sort: readSearchSort(
            params,
            defaultSearchAuthUserAccountSort,
            readSearchAuthUserAccountSortKey,
        ),
        loginId: readSearchTextFilter(params, filterName("loginId")),
        granted: toGranted(readSearchMultipleValueFilter(params, filterName("granted"))),
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
    filter: SearchAuthUserAccountFilter,
): URL {
    let url = new URL(currentURL.toString())
    for (const key of Object.keys(filter)) {
        url = updateQuery(url, key as keyof SearchAuthUserAccountFilter)
    }
    return url

    function updateQuery(url: URL, key: keyof SearchAuthUserAccountFilter): URL {
        switch (key) {
            case "offset":
                return updateSearchOffset(url, filter.offset)

            case "sort":
                return updateSearchSort(url, filter.sort)

            case "loginId":
                return updateSearchTextFilter(url, filterName(key), filter[key])

            case "granted":
                return updateSearchMultipleValueFilter(url, filterName(key), filter[key])
        }
    }
}
export function updateFocusAuthUserAccountQuery(currentURL: URL, user: AuthUserAccount): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.set(FOCUS_ID, user.loginId)
    return url
}
export function clearFocusAuthUserAccountQuery(currentURL: URL): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.delete(FOCUS_ID)
    return url
}

function filterName(key: keyof SearchAuthUserAccountFilterProps): string {
    return `filter-${key}`
}
