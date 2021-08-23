import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockInputLoginIDAction } from "../../../login_id/_ui/action_input/mock"
import { mockInputPasswordAction } from "../action_input/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { initSignLink } from "../../../_ui/common/nav/action_nav/init"

import {
    AuthenticatePasswordAction,
    AuthenticatePasswordState,
    initialAuthenticatePasswordState,
    ValidateAuthenticatePasswordFieldsAction,
} from "./action"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockAuthenticatePasswordAction(): AuthenticatePasswordAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<AuthenticatePasswordState>
    implements AuthenticatePasswordAction
{
    readonly initialState = initialAuthenticatePasswordState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly password = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate: ValidateAuthenticatePasswordFieldsAction = mockValidateBoardAction()

    clear(): void {
        return
    }
    async submit(): Promise<AuthenticatePasswordState> {
        return this.initialState
    }
    async loadError(): Promise<AuthenticatePasswordState> {
        return this.initialState
    }
}
