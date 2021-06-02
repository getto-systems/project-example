import { ApplicationAction } from "../../../../../../ui/vendor/getto-application/action/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { InputLoginIDAction } from "../../../../login_id/_ui/action_input/core/action"
import { InputPasswordAction } from "../../action_input/core/action"

import { AuthenticatePasswordFields } from "../../authenticate/data"

export interface AuthenticatePasswordFormAction extends ApplicationAction {
    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateAuthenticatePasswordFieldsAction
    readonly clear: ClearAction
}

export enum AuthenticatePasswordFieldsEnum {
    "loginID" = "loginID",
    "password" = "password",
}
export type ValidateAuthenticatePasswordFieldsAction = ValidateBoardAction<
    keyof typeof AuthenticatePasswordFieldsEnum,
    AuthenticatePasswordFields
>

interface ClearAction {
    (): void
}
