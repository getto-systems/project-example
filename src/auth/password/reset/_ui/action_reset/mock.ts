import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"

import { initSignLink } from "../../../../_ui/common/nav/action_nav/init"
import { mockInputLoginIDAction } from "../../../../login_id/_ui/action_input/mock"
import { mockInputPasswordAction } from "../../../_ui/action_input/mock"
import { mockValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import {
    initialResetPasswordState,
    ResetPasswordAction,
    ResetPasswordState,
    ValidateResetAction,
} from "./action"
import { emptyBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockResetPasswordAction(): ResetPasswordAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<ResetPasswordState> implements ResetPasswordAction {
    readonly initialState = initialResetPasswordState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly password = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate: ValidateResetAction = mockValidateBoardAction()

    clear(): void {
        return
    }
    async submit(): Promise<ResetPasswordState> {
        return this.initialState
    }
    async loadError(): Promise<ResetPasswordState> {
        return this.initialState
    }
}
