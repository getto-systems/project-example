import { ApplicationView } from "../../../../../../ui/vendor/getto-application/action/action"

import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { ResetPasswordAction, ResetPasswordState } from "./action"

export type ResetPasswordView = ApplicationView<ResetPasswordAction>

export type ResetPasswordResource = Readonly<{
    reset: ResetPasswordAction
}>

export type ResetPasswordResourceState = Readonly<{
    state: ResetPasswordState
    validate: ValidateBoardActionState
}>
