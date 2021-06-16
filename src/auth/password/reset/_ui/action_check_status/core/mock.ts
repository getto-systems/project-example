import { ApplicationMockStateAction } from "../../../../../../../ui/vendor/getto-application/action/mock"

import {
    CheckResetTokenSendingStatusCoreAction,
    CheckResetTokenSendingStatusCoreState,
    initialCheckResetTokenSendingStatusCoreState,
} from "./action"

export function mockCheckResetTokenSendingStatusCoreAction(): CheckResetTokenSendingStatusCoreAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<CheckResetTokenSendingStatusCoreState>
    implements CheckResetTokenSendingStatusCoreAction {
    readonly initialState = initialCheckResetTokenSendingStatusCoreState
}
