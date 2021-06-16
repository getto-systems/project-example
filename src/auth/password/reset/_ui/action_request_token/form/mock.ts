import { mockInputLoginIDAction } from "../../../../../login_id/_ui/action_input/core/mock"
import { mockValidateBoardAction } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/core/mock"

import { RequestResetTokenFormAction } from "./action"

export function mockRequestResetTokenFormAction(): RequestResetTokenFormAction {
    return {
        loginID: mockInputLoginIDAction(),
        validate: mockValidateBoardAction(),
        clear: () => null,
        terminate: () => null,
    }
}
