import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { LogoutMethod } from "../../logout/method"

import { LogoutEvent } from "../../logout/event"

export interface LogoutCoreAction extends ApplicationStateAction<LogoutCoreState> {
    submit(): Promise<LogoutCoreState>
}

export type LogoutCoreMaterial = Readonly<{
    clear: LogoutMethod
}>

export type LogoutCoreState = Readonly<{ type: "initial-logout" }> | LogoutEvent

export const initialLogoutCoreState: LogoutCoreState = { type: "initial-logout" }
