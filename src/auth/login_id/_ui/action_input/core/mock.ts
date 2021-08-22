import { mockValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/mock"
import { initInputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/impl"

import { InputLoginIDAction } from "./action"

export function mockInputLoginIDAction(): InputLoginIDAction {
    const { input, subscriber } = initInputBoardAction()

    return {
        input,
        validate: mockValidateBoardFieldAction("loginID", { valid: false, err: [] }),
        clear: () => null,
        terminate: () => {
            subscriber.terminate()
        },
    }
}
