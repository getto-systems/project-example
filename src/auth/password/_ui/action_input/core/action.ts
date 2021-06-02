import { InputBoardValueResource } from "../../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../../ui/vendor/getto-application/board/action_validate_field/core/action"

import { Password, PasswordCharacterState, ValidatePasswordError } from "../../data"
import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"

export interface InputPasswordAction extends ApplicationAction {
    readonly board: InputBoardValueResource
    readonly validate: ValidatePasswordAction
    readonly clear: ClearAction
    readonly checkCharacter: CheckPasswordCharacterAction
}

export type ValidatePasswordAction = ValidateBoardFieldAction<Password, ValidatePasswordError>
export type ValidatePasswordState = ValidateBoardFieldState<ValidatePasswordError>

interface ClearAction {
    (): void
}
interface CheckPasswordCharacterAction {
    (): PasswordCharacterState
}
