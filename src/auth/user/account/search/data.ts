import { BoardValue } from "../../../../z_vendor/getto-application/board/kernel/data"
import { SingleValueFilter, SearchPageResponse } from "../../../../z_lib/ui/search/kernel/data"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccountBasket } from "../kernel/data"

export type SearchAuthUserAccountFilter = Readonly<{
    offset: BoardValue
    sort: SearchAuthUserAccountSort
    loginID: SingleValueFilter
}>

export type SearchAuthUserAccountSort = SearchSort<"login-id">
export type SearchAuthUserAccountSortKey = SearchAuthUserAccountSort["key"]

export const defaultSearchAuthUserAccountSort: SearchAuthUserAccountSortKey = "login-id"

export type SearchAuthUserAccountRemoteResponse = Readonly<{
    page: SearchPageResponse
    users: readonly AuthUserAccountBasket[]
}>
