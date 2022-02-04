import { readSearchParams } from "../../../../z_vendor/getto-application/board/kernel/convert"
import {
    readSearchOffset,
    readSearchSort,
    updateSearchQuery,
} from "../../../../z_lib/ui/search/convert"

import { SearchAuthUserAccountFieldsDetectParams } from "./infra"

import { SearchAuthUserAccountFields } from "./data"

const SEARCH_LOGIN_ID = "login-id" as const

export function detectSearchAuthUserAccountFields(
    currentURL: URL,
    { defaultSortKey }: SearchAuthUserAccountFieldsDetectParams,
): SearchAuthUserAccountFields {
    const params = currentURL.searchParams
    return {
        offset: readSearchOffset(params),
        sort: readSearchSort(params, { key: defaultSortKey, order: "normal" }),
        loginID: readSearchParams(params, { name: SEARCH_LOGIN_ID }),
    }
}
export function updateSearchAuthUserAccountFieldsQuery(
    currentURL: URL,
    fields: SearchAuthUserAccountFields,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_LOGIN_ID, fields.loginID)

    return updateSearchQuery(url, fields.offset, fields.sort)
}
