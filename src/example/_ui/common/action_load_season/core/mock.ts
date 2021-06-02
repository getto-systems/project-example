import { initialLoadSeasonCoreState, LoadSeasonCoreAction, LoadSeasonCoreState } from "./action"

import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"

export function mockLoadSeasonCoreAction(): LoadSeasonCoreAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<LoadSeasonCoreState> {
    initialState = initialLoadSeasonCoreState
}
