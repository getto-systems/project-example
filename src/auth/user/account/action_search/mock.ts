import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockObserveBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_observe_field/mock"
import { mockSearchColumnsAction } from "../../../../z_lib/ui/search/action_columns/mock"
import { mockSearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/mock"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { mockSearchLoginIDAction } from "../../login_id/input/action_search/mock"

import {
    SearchAuthUserAccountAction,
    SearchAuthUserAccountState,
    initialSearchAuthUserAccountState,
} from "./action"

export function mockSearchAuthUserAccountAction(): SearchAuthUserAccountAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<SearchAuthUserAccountState>
    implements SearchAuthUserAccountAction
{
    readonly initialState = initialSearchAuthUserAccountState

    readonly loginID = mockSearchLoginIDAction()
    readonly offset = mockSearchOffsetAction()
    readonly columns = mockSearchColumnsAction()
    readonly observe = mockObserveBoardFieldAction()

    currentSort(): SearchSort {
        return { key: "login-id", order: "normal" }
    }

    clear(): SearchAuthUserAccountState {
        return this.initialState
    }
    async submit(): Promise<SearchAuthUserAccountState> {
        return this.initialState
    }
    async load(): Promise<SearchAuthUserAccountState> {
        return this.initialState
    }
    async sort(): Promise<SearchAuthUserAccountState> {
        return this.initialState
    }
}
