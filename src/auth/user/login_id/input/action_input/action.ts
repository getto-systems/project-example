import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { InputBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_input/action"
import {
    ValidateBoardFieldAction,
    ValidateBoardFieldState,
} from "../../../../../../ui/vendor/getto-application/board/action_validate_field/action"

import { ValidateLoginIDError } from "../data"

export interface InputLoginIDAction extends ApplicationAction {
    readonly input: InputBoardAction
    readonly validate: ValidateLoginIDAction
    clear(): void
}

export type ValidateLoginIDAction = ValidateBoardFieldAction<ValidateLoginIDError>
export type ValidateLoginIDState = ValidateBoardFieldState<ValidateLoginIDError>
