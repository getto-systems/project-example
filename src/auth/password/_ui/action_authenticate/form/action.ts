import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { InputLoginIDAction } from "../../../../login_id/_ui/action_input/action"
import { InputPasswordAction } from "../../action_input/core/action"

import { AuthenticatePasswordFields } from "../../authenticate/data"

export interface AuthenticatePasswordFormAction extends ApplicationAction {
    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateAuthenticatePasswordFieldsAction
    readonly clear: ClearAction
}

export const authenticatePasswordFields = ["loginID", "password"] as const
export type ValidateAuthenticatePasswordFieldsAction = ValidateBoardAction<
    typeof authenticatePasswordFields[number],
    AuthenticatePasswordFields
>

interface ClearAction {
    (): void
}
