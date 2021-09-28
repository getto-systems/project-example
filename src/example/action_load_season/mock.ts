import { initialLoadSeasonState, LoadSeasonAction, LoadSeasonState } from "./action"

import { ApplicationMockStateAction } from "../../../ui/vendor/getto-application/action/mock"

export function mockLoadSeasonAction(): LoadSeasonAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<LoadSeasonState> {
    initialState = initialLoadSeasonState
}
