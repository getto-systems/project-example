import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"
import { mockValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/mock"
import { mockInputLoginIDAction } from "../../../login_id/input/action_input/mock"
import { initSignLink } from "../../../../sign/action_nav/init"

import {
    initialRequestResetTokenProfileState,
    RequestResetTokenProfileAction,
    RequestResetTokenProfileState,
} from "./action"

export function mockRequestResetTokenProfileAction(): RequestResetTokenProfileAction {
    return new Action()
}

export class Action
    extends ApplicationMockStateAction<RequestResetTokenProfileState>
    implements RequestResetTokenProfileAction
{
    readonly initialState = initialRequestResetTokenProfileState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly validate = mockValidateBoardAction()

    open(): RequestResetTokenProfileState {
        return this.initialState
    }
    clear(): RequestResetTokenProfileState {
        return this.initialState
    }
    async submit(): Promise<RequestResetTokenProfileState> {
        return this.initialState
    }
    close(): RequestResetTokenProfileState {
        return this.initialState
    }
}
