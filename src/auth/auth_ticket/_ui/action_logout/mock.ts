import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { LogoutResource } from "./resource"

import { initialLogoutState, LogoutAction, LogoutState } from "./action"

export function mockLogoutResource(): LogoutResource {
    return { logout: mockLogoutAction() }
}

export function mockLogoutAction(): LogoutAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<LogoutState> implements LogoutAction {
    readonly initialState = initialLogoutState
    async submit(): Promise<LogoutState> {
        return this.initialState
    }
}
