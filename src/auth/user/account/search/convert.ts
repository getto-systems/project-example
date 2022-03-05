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
import { DetectLoginIDResult } from "./infra"
import { AuthUserAccountBasket } from "../kernel/data"

const SEARCH_LOGIN_ID = "search-login-id" as const

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
        loginID: readSingleValueFilter(params, SEARCH_LOGIN_ID),
    }
}
export function readSearchAuthUserAccountSortKey(
    key: string,
): ReadSearchSortKeyResult<SearchAuthUserAccountSortKey> {
    switch (key) {
        case "login-id":
            return { found: true, key }

        default:
            return { found: false }
    }
}

export function detectFocusAuthUserAccount(currentURL: URL): DetectLoginIDResult {
    const loginID = currentURL.searchParams.get(FOCUS_ID)
    if (loginID === null) {
        return { found: false }
    }
    return { found: true, loginID }
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
export function updateFocusAuthUserAccountQuery(currentURL: URL, user: AuthUserAccountBasket): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.set(FOCUS_ID, user.loginID)
    return url
}
export function clearFocusAuthUserAccountQuery(currentURL: URL): URL {
    const url = new URL(currentURL.toString())
    url.searchParams.delete(FOCUS_ID)
    return url
}
