import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../../ui/vendor/getto-application/board/action_validate_field/action"

import { LoginID, ValidateLoginIDError } from "../../data"

export interface InputLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidateLoginIDAction
    readonly clear: ClearAction
}

export type ValidateLoginIDAction = ValidateBoardFieldAction<LoginID, ValidateLoginIDError>
export type ValidateLoginIDState = ValidateBoardFieldState<ValidateLoginIDError>

interface ClearAction {
    (): void
}
