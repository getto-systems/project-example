import { TextFilter, SearchPageResponse } from "../../../../common/util/search/kernel/data"
import { SearchSort } from "../../../../common/util/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { AuthPermission } from "../../kernel/data"
import { ListSearchedData } from "../../../../common/util/list/data"

export type SearchAuthUserAccountFilter = SearchAuthUserAccountFilterProps &
    Readonly<{
        offset: string
        sort: SearchAuthUserAccountSort
    }>
export type SearchAuthUserAccountFilterProps = Readonly<{
    loginId: TextFilter
    granted: readonly AuthPermission[]
}>

export const searchAuthUserAccountSortKeys = ["loginId"] as const
export type SearchAuthUserAccountSort = SearchSort<typeof searchAuthUserAccountSortKeys[number]>
export type SearchAuthUserAccountSortKey = SearchAuthUserAccountSort["key"]

export const defaultSearchAuthUserAccountSort = searchAuthUserAccountSortKeys[0]

export type SearchAuthUserAccountRemoteResponse = ListSearchedData<
    AuthUserAccount,
    SearchAuthUserAccountSummary
>
export type SearchAuthUserAccountSummary = Readonly<{
    page: SearchPageResponse
    sort: SearchAuthUserAccountSort
}>
