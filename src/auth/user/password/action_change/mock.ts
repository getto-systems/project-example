import { ApplicationMockStateAction } from "../../../../../ui/vendor/getto-application/action/mock"

import { mockInputPasswordAction } from "../input/action_input/mock"
import { mockValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { ChangePasswordAction, ChangePasswordState, initialChangePasswordState } from "./action"

import { emptyBoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockChangePasswordAction(): ChangePasswordAction {
    return new Action()
}

class Action
    extends ApplicationMockStateAction<ChangePasswordState>
    implements ChangePasswordAction
{
    readonly initialState = initialChangePasswordState

    readonly currentPassword = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly newPassword = mockInputPasswordAction(emptyBoardValue, { multiByte: false })
    readonly validate = mockValidateBoardAction()

    open(): ChangePasswordState {
        return this.initialState
    }
    clear(): ChangePasswordState {
        return this.initialState
    }
    async submit(): Promise<ChangePasswordState> {
        return this.initialState
    }
    close(): ChangePasswordState {
        return this.initialState
    }
}
