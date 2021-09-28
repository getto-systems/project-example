import { ApplicationMockStateAction } from "../../../../ui/vendor/getto-application/action/mock"

import {
    FindNextVersionState,
    FindNextVersionAction,
    initialFindNextVersionState,
} from "./action"

export function mockFindNextVersionAction(): FindNextVersionAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<FindNextVersionState>
    implements FindNextVersionAction {
    readonly initialState = initialFindNextVersionState
}
