import { readSearchParams } from "../../../../../ui/vendor/getto-application/board/kernel/convert"

import { SearchUserAccountFields } from "./data"

const SEARCH_LOGIN_ID = "login-id" as const

export function detectSearchUserAccountFields(currentURL: URL): SearchUserAccountFields {
    const params = currentURL.searchParams
    return {
        loginID: readSearchParams(params, SEARCH_LOGIN_ID),
    }
}
export function updateSearchUserAccountFieldsQuery(
    currentURL: URL,
    fields: SearchUserAccountFields,
): URL {
    const url = new URL(currentURL.toString())
    const params = url.searchParams
    params.set(SEARCH_LOGIN_ID, fields.loginID)
    return url
}
