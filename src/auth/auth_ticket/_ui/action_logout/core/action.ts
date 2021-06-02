import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { ClearAuthTicketMethod } from "../../clear/method"

import { ClearAuthTicketEvent } from "../../clear/event"

export interface LogoutCoreAction extends ApplicationStateAction<LogoutCoreState> {
    submit(): Promise<LogoutCoreState>
}

export type LogoutCoreMaterial = Readonly<{
    clear: ClearAuthTicketMethod
}>

export type LogoutCoreState = Readonly<{ type: "initial-logout" }> | ClearAuthTicketEvent

export const initialLogoutCoreState: LogoutCoreState = { type: "initial-logout" }
