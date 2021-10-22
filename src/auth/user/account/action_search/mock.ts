import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockObserveBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_observe_field/mock"
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
    readonly observe = mockObserveBoardFieldAction()

    clear(): SearchUserAccountState {
        return this.initialState
    }
    async submit(): Promise<SearchUserAccountState> {
        return this.initialState
    }
}
