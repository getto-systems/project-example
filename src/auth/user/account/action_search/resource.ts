import { ObserveBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { SearchColumnsState } from "../../../../z_lib/ui/search/action_columns/action"
import { SearchUserAccountAction, SearchUserAccountState } from "./action"

export type SearchUserAccountResource = Readonly<{
    search: SearchUserAccountAction
}>

export type SearchUserAccountFormResourceState = Readonly<{
    state: SearchUserAccountState
    observe: ObserveBoardActionState
}>
export type SearchUserAccountPagerResourceState = Readonly<{
    state: SearchUserAccountState
}>
export type SearchUserAccountColumnsResourceState = Readonly<{
    columns: SearchColumnsState
}>
export type SearchUserAccountTableResourceState = Readonly<{
    state: SearchUserAccountState
    columns: SearchColumnsState
}>
