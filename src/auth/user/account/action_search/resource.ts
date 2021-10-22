import { ObserveBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { SearchUserAccountAction, SearchUserAccountState } from "./action"

export type SearchUserAccountResource = Readonly<{
    search: SearchUserAccountAction
}>

export type SearchUserAccountResourceState = Readonly<{
    state: SearchUserAccountState
    observe: ObserveBoardActionState
}>
