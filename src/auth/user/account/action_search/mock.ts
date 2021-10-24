import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockObserveBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_observe_field/mock"
import { mockSearchColumnsAction } from "../../../../z_lib/ui/search/action_columns/mock"
import { mockSearchOffsetAction } from "../../../../z_lib/ui/search/action_offset/mock"
import { SearchSort } from "../../../../z_lib/ui/search/sort/data"
import { mockSearchLoginIDAction } from "../../login_id/input/action_search/mock"

import {
    SearchUserAccountAction,
    SearchUserAccountState,
    initialSearchUserAccountState,
} from "./action"

export function mockSearchUserAccountAction(): SearchUserAccountAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<SearchUserAccountState>
    implements SearchUserAccountAction
{
    readonly initialState = initialSearchUserAccountState

    readonly loginID = mockSearchLoginIDAction()
    readonly offset = mockSearchOffsetAction()
    readonly columns = mockSearchColumnsAction()
    readonly observe = mockObserveBoardFieldAction()

    currentSort(): SearchSort {
        return { key: "login-id", order: "normal" }
    }

    clear(): SearchUserAccountState {
        return this.initialState
    }
    async submit(): Promise<SearchUserAccountState> {
        return this.initialState
    }
    async load(): Promise<SearchUserAccountState> {
        return this.initialState
    }
    async sort(): Promise<SearchUserAccountState> {
        return this.initialState
    }
}
