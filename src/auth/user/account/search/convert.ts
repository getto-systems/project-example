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
import { DetectLoginIdResult } from "./infra"
import { AuthUserAccount } from "../kernel/data"
import { toGrantedRoles } from "../input/granted_roles/convert"

const SEARCH_LOGIN_ID = "search-login-id" as const
const SEARCH_GRANTED_ROLES = "search-granted-roles" as const

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
        loginId: readSingleValueFilter(params, SEARCH_LOGIN_ID),
        grantedRoles: toGrantedRoles(readMultipleValueFilter(params, SEARCH_GRANTED_ROLES)),
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

export function detectFocusAuthUserAccount(currentURL: URL): DetectLoginIdResult {
    const loginId = currentURL.searchParams.get(FOCUS_ID)
    if (loginId === null) {
        return { found: false }
    }
    return { found: true, loginId }
}

export function updateSearchAuthUserAccountFilterQuery(
    currentURL: URL,
    fields: SearchAuthUserAccountFilter,
): URL {
    let url = new URL(currentURL.toString())
    url = updateSingleValueFilter(url, SEARCH_LOGIN_ID, fields.loginId)
    url = updateMultipleValueFilter(url, SEARCH_GRANTED_ROLES, fields.grantedRoles)
    url = updateSearchOffset(url, fields.offset)
    url = updateSearchSort(url, fields.sort)
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
