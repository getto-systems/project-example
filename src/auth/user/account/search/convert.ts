import { readSearchOffset, updateSearchOffset } from "../../../../z_lib/ui/search/offset/convert"
import { readSearchSort, updateSearchSort } from "../../../../z_lib/ui/search/sort/convert"
import {
    readSingleValueFilter,
    updateSingleValueFilter,
} from "../../../../z_lib/ui/search/kernel/convert"

import {
    defaultSearchAuthUserAccountSort,
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountSortKey,
} from "./data"
import { ReadSearchSortKeyResult } from "../../../../z_lib/ui/search/sort/data"

const SEARCH_LOGIN_ID = "login-id" as const

export function detectSearchAuthUserAccountFilter(currentURL: URL): SearchAuthUserAccountFilter {
    const params = currentURL.searchParams
    return {
        offset: readSearchOffset(params),
        sort: readSearchSort(
            params,
            defaultSearchAuthUserAccountSort,
            (value: string): ReadSearchSortKeyResult<SearchAuthUserAccountSortKey> => {
                switch (value) {
                    case "login-id":
                        return { found: true, key: value }

                    default:
                        return { found: false }
                }
            },
        ),
        loginID: readSingleValueFilter(params, SEARCH_LOGIN_ID),
    }
}
export function updateSearchAuthUserAccountFilterQuery(
    currentURL: URL,
    fields: SearchAuthUserAccountFilter,
): URL {
    let url = new URL(currentURL.toString())
    url = updateSingleValueFilter(url, SEARCH_LOGIN_ID, fields.loginID)
    url = updateSearchOffset(url, fields.offset)
    url = updateSearchSort(url, fields.sort)
    return url
}
