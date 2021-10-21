import { ApplicationMockStateAction } from "../../../../../../ui/vendor/getto-application/action/mock"

import { initSignLink } from "../../../../sign/action_nav/init"
import { mockInputLoginIDAction } from "../../../login_id/input/action_input/mock"
import { mockInputPasswordAction } from "../../input/action_input/mock"
import { mockValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { initialResetPasswordState, ResetPasswordAction, ResetPasswordState } from "./action"
import { emptyBoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockResetPasswordAction(): ResetPasswordAction {
    return new Action()
}

class Action extends ApplicationMockStateAction<ResetPasswordState> implements ResetPasswordAction {
    readonly initialState = initialResetPasswordState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly password = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate = mockValidateBoardAction()

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
