import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockInputLoginIDAction } from "../../login_id/input/action_input/mock"
import { mockInputPasswordAction } from "../../password/input/action_input/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { initSignLink } from "../../../sign/action_nav/init"

import {
    ManageUserAccountAction,
    ManageUserAccountState,
    initialManageUserAccountState,
} from "./action"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockAuthenticatePasswordAction(): ManageUserAccountAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<ManageUserAccountState>
    implements ManageUserAccountAction
{
    readonly initialState = initialManageUserAccountState

    readonly link = initSignLink()

    readonly loginID = mockInputLoginIDAction()
    readonly password = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate = mockValidateBoardAction()

    clear(): ManageUserAccountState {
        return this.initialState
    }
    async submit(): Promise<ManageUserAccountState> {
        return this.initialState
    }
    async loadError(): Promise<ManageUserAccountState> {
        return this.initialState
    }
}
