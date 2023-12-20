import { SearchPageResponse } from "../../../../common/util/search/kernel/data"
import { SearchSort } from "../../../../common/util/search/sort/data"
import { AuthUserAccount } from "../kernel/data"
import { AuthPermission } from "../../kernel/data"
import { ListSearchedData } from "../../../../common/util/list/data"
import {
    MultipleFilterBoardFilter,
    SingleFilterBoardValue,
} from "../../../../common/util/board/filter/data"

export type SearchAuthUserAccountFilterData = Readonly<{
    offset: string
    sort: SearchAuthUserAccountSort
    filter: SearchAuthUserAccountFilter
}>
export type SearchAuthUserAccountFilter = Readonly<{
    loginId: SingleFilterBoardValue
    granted: MultipleFilterBoardFilter<AuthPermission>
}>

export const searchAuthUserAccountSortKeys = ["loginId"] as const
export type SearchAuthUserAccountSortKey = (typeof searchAuthUserAccountSortKeys)[number]
export type SearchAuthUserAccountSort = SearchSort<SearchAuthUserAccountSortKey>

export const defaultSearchAuthUserAccountSort = searchAuthUserAccountSortKeys[0]

export type SearchAuthUserAccountRemoteResponse = ListSearchedData<
    AuthUserAccount,
    SearchAuthUserAccountSummary
>
export type SearchAuthUserAccountSummary = Readonly<{
    page: SearchPageResponse
    sort: SearchAuthUserAccountSort
}>
