import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { SearchOffsetAction } from "../../../../z_lib/ui/remote/search/action_search/action"

import { SearchUserAccountMethod } from "../search/method"

import { SearchUserAccountEvent } from "../search/event"

export interface SearchUserAccountAction extends ApplicationStateAction<SearchUserAccountState> {
    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly observe: ObserveBoardAction

    clear(): SearchUserAccountState
    submit(): Promise<SearchUserAccountState>
    load(): Promise<SearchUserAccountState>
}

export const searchUserAccountFieldNames = ["loginID"] as const
export type SearchUserAccountFieldName = typeof searchUserAccountFieldNames[number]

export type SearchUserAccountMaterial = Readonly<{
    search: SearchUserAccountMethod
}>

export type SearchUserAccountState = Readonly<{ type: "initial-search" }> | SearchUserAccountEvent

export const initialSearchUserAccountState: SearchUserAccountState = { type: "initial-search" }
