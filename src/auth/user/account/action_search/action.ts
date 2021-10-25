import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { SearchLoginIDAction } from "../../login_id/input/action_search/action"
import { ObserveBoardAction } from "../../../../../ui/vendor/getto-application/board/action_observe_board/action"
import { SearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/action"
import { SearchColumnsAction, SearchColumnsMaterial } from "../../../../z_lib/ui/search/action_columns/action"

import { SearchAuthUserAccountMethod as SearchAuthUserAccountMethod } from "../search/method"

import { SearchAuthUserAccountEvent } from "../search/event"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"

export interface SearchAuthUserAccountAction extends ApplicationStateAction<SearchAuthUserAccountState> {
    readonly loginID: SearchLoginIDAction
    readonly offset: SearchOffsetAction
    readonly columns: SearchColumnsAction
    readonly observe: ObserveBoardAction

    currentSort(): SearchSort

    clear(): SearchAuthUserAccountState
    submit(): Promise<SearchAuthUserAccountState>
    load(): Promise<SearchAuthUserAccountState>
    sort(key: string): Promise<SearchAuthUserAccountState>
}

export const searchAuthUserAccountFieldNames = ["loginID"] as const
export type SearchAuthUserAccountFieldName = typeof searchAuthUserAccountFieldNames[number]

export type SearchAuthUserAccountMaterial = Readonly<{
    search: SearchAuthUserAccountMethod
    columns: SearchColumnsMaterial
}>

export type SearchAuthUserAccountState = Readonly<{ type: "initial-search" }> | SearchAuthUserAccountEvent

export const initialSearchAuthUserAccountState: SearchAuthUserAccountState = { type: "initial-search" }
