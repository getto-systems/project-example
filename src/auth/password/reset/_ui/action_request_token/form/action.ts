import { ApplicationAction } from "../../../../../../../ui/vendor/getto-application/action/action"
import { ValidateBoardAction } from "../../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { InputLoginIDAction } from "../../../../../login_id/_ui/action_input/action"

import { RequestResetTokenFields } from "../../request_token/data"

export interface RequestResetTokenFormAction extends ApplicationAction {
    readonly loginID: InputLoginIDAction
    readonly validate: ValidateRequestTokenAction
    readonly clear: ClearAction
}

export const requestResetTokenFields = ["loginID"] as const
export type ValidateRequestTokenAction = ValidateBoardAction<
    typeof requestResetTokenFields[number],
    RequestResetTokenFields
>

interface ClearAction {
    (): void
}
