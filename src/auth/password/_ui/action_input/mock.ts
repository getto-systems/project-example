import { mockValidateBoardFieldAction } from "../../../../../ui/vendor/getto-application/board/action_validate_field/mock"
import { initInputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/init"

import { InputPasswordAction } from "./action"

import { BoardValue } from "../../../../../ui/vendor/getto-application/board/kernel/data"
import { PasswordCharacterState } from "../data"

export function mockInputPasswordAction(
    password: BoardValue,
    characterState: PasswordCharacterState,
): InputPasswordAction {
    const { input, subscriber } = initInputBoardAction()

    return {
        input,
        validate: mockValidateBoardFieldAction("password", { valid: false, err: [] }),
        clear: () => null,
        checkCharacter: () => characterState,
        terminate: () => {
            subscriber.terminate()
        },
    }
}
