import { InputBoardAction } from "../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../ui/vendor/getto-application/board/action_validate_field/action"

import { Password, PasswordCharacterState, ValidatePasswordError } from "../data"
import { ApplicationAction } from "../../../../../ui/vendor/getto-application/action/action"

export interface InputPasswordAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidatePasswordAction
    clear(): void
    checkCharacter(): PasswordCharacterState
}

export type ValidatePasswordAction = ValidateBoardFieldAction<Password, ValidatePasswordError>
export type ValidatePasswordState = ValidateBoardFieldState<ValidatePasswordError>