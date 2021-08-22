import { mockInputLoginIDAction } from "../../../../../login_id/_ui/action_input/core/mock"
import { mockInputPasswordAction } from "../../../../_ui/action_input/core/mock"
import { mockValidateBoardAction } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/mock"

import { ResetPasswordFormAction } from "./action"

import { emptyBoardValue } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"

export function mockResetPasswordFormAction(): ResetPasswordFormAction {
    return {
        loginID: mockInputLoginIDAction(),
        password: mockInputPasswordAction(emptyBoardValue, { multiByte: false }),
        validate: mockValidateBoardAction(),
        clear: () => null,
        terminate: () => null,
    }
}
