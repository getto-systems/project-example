import { readSearchOffset, updateSearchOffset } from "../../../../z_lib/ui/search/offset/convert"
import { readSearchSort, updateSearchSort } from "../../../../z_lib/ui/search/sort/convert"
import {
    readMultipleValueFilter,
    readSingleValueFilter,
    updateMultipleValueFilter,
    updateSingleValueFilter,
} from "../../../../z_lib/ui/search/kernel/convert"

import {
    defaultSearchAuthUserAccountSort,
    SearchAuthUserAccountFilter,
    SearchAuthUserAccountSortKey,
} from "./data"
import { ReadSearchSortKeyResult } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { toGrantedRoles } from "../input/granted_roles/convert"
import { DetectFocusListKeyResult } from "../../../../z_lib/ui/list/data"

const FILTER_LOGIN_ID = "filter-login-id" as const
const FILTER_GRANTED_ROLES = "filter-granted-roles" as const

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
        loginId: readSingleValueFilter(params, FILTER_LOGIN_ID),
        grantedRoles: toGrantedRoles(readMultipleValueFilter(params, FILTER_GRANTED_ROLES)),
    }
}
export function readSearchAuthUserAccountSortKey(
    key: string,
): ReadSearchSortKeyResult<SearchAuthUserAccountSortKey> {
    switch (key) {
        case "loginId":
            return { found: true, key }

        default:
            return { found: false }
    }
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
    url = updateSingleValueFilter(url, FILTER_LOGIN_ID, filter.loginId)
    url = updateMultipleValueFilter(url, FILTER_GRANTED_ROLES, filter.grantedRoles)
    url = updateSearchOffset(url, filter.offset)
    url = updateSearchSort(url, filter.sort)
    return url
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
