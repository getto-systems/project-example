import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { ValidateBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { AuthenticatePasswordAction, AuthenticatePasswordState } from "./action"

export type AuthenticatePasswordView = ApplicationView<AuthenticatePasswordAction>

export type AuthenticatePasswordResource = Readonly<{
    authenticate: AuthenticatePasswordAction
}>

export type AuthenticatePasswordResourceState = Readonly<{
    state: AuthenticatePasswordState
    validate: ValidateBoardActionState
}>
