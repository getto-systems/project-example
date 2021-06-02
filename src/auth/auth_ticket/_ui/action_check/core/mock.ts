import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"

import {
    CheckAuthTicketCoreAction,
    CheckAuthTicketCoreState,
    initialCheckAuthTicketCoreState,
} from "./action"

export function mockCheckAuthTicketCoreAction(): CheckAuthTicketCoreAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<CheckAuthTicketCoreState>
    implements CheckAuthTicketCoreAction {
    readonly initialState = initialCheckAuthTicketCoreState

    constructor() {
        super(async () => ({ type: "required-to-login" }))
    }

    async succeedToInstantLoad(): Promise<CheckAuthTicketCoreState> {
        return this.initialState
    }
    async failedToInstantLoad(): Promise<CheckAuthTicketCoreState> {
        return this.initialState
    }
    async loadError(): Promise<CheckAuthTicketCoreState> {
        return this.initialState
    }
}
