import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { CheckAuthTicketCoreAction, CheckAuthTicketCoreState } from "./core/action"

export type CheckAuthTicketView = ApplicationView<CheckAuthTicketResource>

export type CheckAuthTicketResource = Readonly<{
    core: CheckAuthTicketCoreAction
}>
export type CheckAuthTicketResourceState = Readonly<{
    state: CheckAuthTicketCoreState
}>
