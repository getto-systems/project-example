import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"

import { SearchUserAccountMethod } from "../search/method"

import { SearchUserAccountEvent } from "../search/event"

export interface ManageUserAccountAction extends ApplicationStateAction<ManageUserAccountState> {
    readonly loginID: SearchLoginIDAction
    readonly observe: ObserveBoardAction

    clear(): ManageUserAccountState
    submit(): Promise<ManageUserAccountState>
}

export const manageUserAccountFieldNames = ["loginID"] as const
export type ManageUserAccountFieldName = typeof manageUserAccountFieldNames[number]

export type ManageUserAccountMaterial = Readonly<{
    search: SearchUserAccountMethod
}>

export type ManageUserAccountState =
    | Readonly<{ type: "initial-manage-user-account" }>
    | SearchUserAccountEvent

export const initialManageUserAccountState: ManageUserAccountState = {
    type: "initial-manage-user-account",
}
