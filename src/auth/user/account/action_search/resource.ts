import { ObserveBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { SearchColumnsState } from "../../../../z_lib/ui/search/action_columns/action"
import { SearchAuthUserAccountAction, SearchAuthUserAccountState } from "./action"

export type SearchAuthUserAccountResource = Readonly<{
    search: SearchAuthUserAccountAction
}>

export type SearchAuthUserAccountFormResourceState = Readonly<{
    state: SearchAuthUserAccountState
    observe: ObserveBoardActionState
}>
export type SearchAuthUserAccountPagerResourceState = Readonly<{
    state: SearchAuthUserAccountState
}>
export type SearchAuthUserAccountColumnsResourceState = Readonly<{
    columns: SearchColumnsState
}>
export type SearchAuthUserAccountTableResourceState = Readonly<{
    state: SearchAuthUserAccountState
    columns: SearchColumnsState
}>
