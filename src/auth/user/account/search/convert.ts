import { readSearchParams } from "../../../../../ui/vendor/getto-application/board/kernel/convert"
import {
    readSearchOffset,
    readSearchSort,
    updateSearchQuery,
} from "../../../../z_lib/ui/search/convert"

import { SearchUserAccountFieldsDetectParams } from "./infra"

import { SearchUserAccountFields } from "./data"

const SEARCH_LOGIN_ID = "login-id" as const

export function detectSearchUserAccountFields(
    currentURL: URL,
    { defaultSortKey }: SearchUserAccountFieldsDetectParams,
): SearchUserAccountFields {
    const params = currentURL.searchParams
    return {
        offset: readSearchOffset(params),
        sort: readSearchSort(params, { key: defaultSortKey, order: "normal" }),
        loginID: readSearchParams(params, { name: SEARCH_LOGIN_ID }),
    }
}
export function updateSearchUserAccountFieldsQuery(
    currentURL: URL,
    fields: SearchUserAccountFields,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_LOGIN_ID, fields.loginID)

    return updateSearchQuery(url, fields.offset, fields.sort)
}
