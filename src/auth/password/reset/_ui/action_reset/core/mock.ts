import { ApplicationMockStateAction } from "../../../../../../../ui/vendor/getto-application/action/mock"

import {
    initialResetPasswordCoreState,
    ResetPasswordCoreAction,
    ResetPasswordCoreState,
} from "./action"

export function mockResetPasswordCoreAction(): ResetPasswordCoreAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<ResetPasswordCoreState>
    implements ResetPasswordCoreAction {
    readonly initialState = initialResetPasswordCoreState

    async submit(): Promise<ResetPasswordCoreState> {
        return this.initialState
    }
    async loadError(): Promise<ResetPasswordCoreState> {
        return this.initialState
    }
}
