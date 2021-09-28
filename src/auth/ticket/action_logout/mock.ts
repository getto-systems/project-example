import { ApplicationMockStateAction } from "../../../../ui/vendor/getto-application/action/mock"

import { initialLogoutState, LogoutAction, LogoutState } from "./action"
import { LogoutResource } from "./resource"

export function mockLogoutResource(): LogoutResource {
    return { logout: new Action() }
}

class Action extends ApplicationMockStateAction<LogoutState> implements LogoutAction {
    readonly initialState = initialLogoutState
    async submit(): Promise<LogoutState> {
        return this.initialState
    }
}
