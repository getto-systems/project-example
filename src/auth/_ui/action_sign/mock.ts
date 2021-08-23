import { ApplicationMockStateAction } from "../../../../ui/vendor/getto-application/action/mock"

import { initialSignViewState, SignAction, SignActionState } from "./action"

export function mockSignAction(): SignAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<SignActionState> implements SignAction {
    readonly initialState = initialSignViewState

    async error(): Promise<SignActionState> {
        return this.initialState
    }
}
