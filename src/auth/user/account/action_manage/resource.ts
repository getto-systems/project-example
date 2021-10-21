import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { ObserveBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { ManageUserAccountAction, ManageUserAccountState } from "./action"

export type ManageUserAccountView = ApplicationView<ManageUserAccountAction>

export type ManageUserAccountResource = Readonly<{
    manage: ManageUserAccountAction
}>

export type ManageUserAccountResourceState = Readonly<{
    state: ManageUserAccountState
    observe: ObserveBoardActionState
}>
