import { ApplicationAction } from "../../../../../../../ui/vendor/getto-application/action/action"
import { InputLoginIDAction } from "../../../../../login_id/_ui/action_input/core/action"
import { InputPasswordAction } from "../../../../_ui/action_input/core/action"
import { ValidateBoardAction } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"

import { ResetPasswordFields } from "../../reset/data"

export interface ResetPasswordFormAction extends ApplicationAction {
    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateResetAction
    readonly clear: ClearAction
}

export enum ResetPasswordFieldsEnum {
    "loginID" = "loginID",
    "password" = "password",
}
export type ValidateResetAction = ValidateBoardAction<
    keyof typeof ResetPasswordFieldsEnum,
    ResetPasswordFields
>

interface ClearAction {
    (): void
}
