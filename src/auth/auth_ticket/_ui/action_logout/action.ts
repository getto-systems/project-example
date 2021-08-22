import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { LogoutMethod } from "../logout/method"

import { LogoutEvent } from "../logout/event"

export interface LogoutAction extends ApplicationStateAction<LogoutState> {
    submit(): Promise<LogoutState>
}

export type LogoutMaterial = Readonly<{
    clear: LogoutMethod
}>

export type LogoutState = Readonly<{ type: "initial-logout" }> | LogoutEvent

export const initialLogoutState: LogoutState = { type: "initial-logout" }
