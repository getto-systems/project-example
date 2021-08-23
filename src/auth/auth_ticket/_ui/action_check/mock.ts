import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { CheckAuthTicketAction, CheckAuthTicketState, initialCheckAuthTicketState } from "./action"

export function mockCheckAuthTicketAction(): CheckAuthTicketAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<CheckAuthTicketState>
    implements CheckAuthTicketAction
{
    readonly initialState = initialCheckAuthTicketState

    constructor() {
        super(async () => ({ type: "required-to-login" }))
    }

    async succeedToInstantLoad(): Promise<CheckAuthTicketState> {
        return this.initialState
    }
    async failedToInstantLoad(): Promise<CheckAuthTicketState> {
        return this.initialState
    }
    async loadError(): Promise<CheckAuthTicketState> {
        return this.initialState
    }
}
