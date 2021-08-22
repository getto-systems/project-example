import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockInputLoginIDAction } from "../../../login_id/_ui/action_input/mock"
import { mockInputPasswordAction } from "../action_input/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { initSignLink } from "../../../_ui/common/nav/action_nav/init"

import { SignLink } from "../../../_ui/common/nav/action_nav/resource"
import {
    AuthenticatePasswordAction,
    AuthenticatePasswordState,
    initialAuthenticatePasswordState,
    ValidateAuthenticatePasswordFieldsAction,
} from "./action"
import { InputLoginIDAction } from "../../../login_id/_ui/action_input/action"
import { InputPasswordAction } from "../action_input/action"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockAuthenticatePasswordAction(): AuthenticatePasswordAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<AuthenticatePasswordState>
    implements AuthenticatePasswordAction
{
    readonly initialState = initialAuthenticatePasswordState

    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateAuthenticatePasswordFieldsAction

    constructor() {
        super()
        this.link = initSignLink()
        this.loginID = mockInputLoginIDAction()
        this.password = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
        this.validate = mockValidateBoardAction()
    }

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
