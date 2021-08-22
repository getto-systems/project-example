import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { LogoutResource } from "./resource"

import { initialLogoutCoreState, LogoutCoreAction, LogoutCoreState } from "./action"

export function mockLogoutResource(): LogoutResource {
    return { logout: mockLogoutCoreAction() }
}

export function mockLogoutCoreAction(): LogoutCoreAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<LogoutCoreState> implements LogoutCoreAction {
    readonly initialState = initialLogoutCoreState
    async submit(): Promise<LogoutCoreState> {
        return this.initialState
    }
}
