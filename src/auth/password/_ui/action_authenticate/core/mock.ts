import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"
import {
    AuthenticatePasswordCoreAction,
    AuthenticatePasswordCoreState,
    initialAuthenticatePasswordCoreState,
} from "./action"

export function mockAuthenticatePasswordCoreAction(): AuthenticatePasswordCoreAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<AuthenticatePasswordCoreState>
    implements AuthenticatePasswordCoreAction {
    readonly initialState = initialAuthenticatePasswordCoreState

    async submit(): Promise<AuthenticatePasswordCoreState> {
        return this.initialState
    }
    async loadError(): Promise<AuthenticatePasswordCoreState> {
        return this.initialState
    }
}
