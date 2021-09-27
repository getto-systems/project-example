import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"
import { mockInputLoginIDAction } from "../../../user/login_id/input/action_input/mock"
import { initSignLink } from "../../../_ui/common/nav/action_nav/init"

import {
    initialRequestResetTokenState,
    RequestResetTokenAction,
    RequestResetTokenState,
} from "./action"

export function mockRequestResetTokenAction(): RequestResetTokenAction {
    return new Action()
}

export class Action
    extends ApplicationMockStateAction<RequestResetTokenState>
    implements RequestResetTokenAction
{
    readonly initialState = initialRequestResetTokenState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly validate = mockValidateBoardAction()

    clear(): void {
        return
    }
    async submit(): Promise<RequestResetTokenState> {
        return this.initialState
    }
}
