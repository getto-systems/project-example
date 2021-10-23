import { VNodeContent } from "../../../../example/x_preact/design/common"

import { ObserveBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
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
    label: { (key: string): VNodeContent }
}>
