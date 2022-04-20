import { SingleValueFilter, SearchPageResponse } from "../../../../z_lib/ui/search/kernel/data"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { AuthRole } from "../../kernel/data"

export type SearchAuthUserAccountFilter = Readonly<{
    offset: string
    sort: SearchAuthUserAccountSort
    loginId: SingleValueFilter
    grantedRoles: readonly AuthRole[]
}>

export type SearchAuthUserAccountSort = SearchSort<"login-id">
export type SearchAuthUserAccountSortKey = SearchAuthUserAccountSort["key"]

export const defaultSearchAuthUserAccountSort: SearchAuthUserAccountSortKey = "login-id"

export type SearchAuthUserAccountRemoteResponse = Readonly<{
    page: SearchPageResponse
    sort: SearchAuthUserAccountSort
    users: readonly AuthUserAccount[]
}>
