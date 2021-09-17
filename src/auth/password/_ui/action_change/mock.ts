import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockInputPasswordAction } from "../action_input/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { ChangePasswordAction, ChangePasswordState, initialChangePasswordState } from "./action"
import { ChangePasswordResource } from "./resource"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockChangePasswordResource(): ChangePasswordResource {
    return { change: new Action() }
}

class Action
    extends ApplicationMockStateAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialChangePasswordState

    readonly currentPassword = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly newPassword = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate = mockValidateBoardAction()

    clear(): void {
        return
    }
    async submit(): Promise<ChangePasswordState> {
        return this.initialState
    }
}
