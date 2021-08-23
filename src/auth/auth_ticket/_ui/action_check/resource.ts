import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { CheckAuthTicketAction, CheckAuthTicketState } from "./action"

export type CheckAuthTicketView = ApplicationView<CheckAuthTicketAction>

export type CheckAuthTicketResource = Readonly<{
    check: CheckAuthTicketAction
}>

export type CheckAuthTicketResourceState = Readonly<{
    state: CheckAuthTicketState
}>
