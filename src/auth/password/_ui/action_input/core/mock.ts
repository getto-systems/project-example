import { mockInputBoardValueResource } from "../../../../../../ui/vendor/getto-application/board/action_input/mock"
import { mockValidateBoardFieldAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/mock"

import { InputPasswordAction } from "./action"

import { BoardValue } from "../../../../../../ui/vendor/getto-application/board/kernel/data"
import { PasswordCharacterState } from "../../data"

export function mockInputPasswordAction(
    password: BoardValue,
    characterState: PasswordCharacterState,
): InputPasswordAction {
    return {
        board: mockInputBoardValueResource("password", password),
        validate: mockValidateBoardFieldAction("password", { valid: false, err: [] }),
        clear: () => null,
        checkCharacter: () => characterState,
        terminate: () => null,
    }
}
