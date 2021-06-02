import {
    ApplicationAction,
    ApplicationView,
} from "../../../../../ui/vendor/getto-application/action/action"

import { SignLinkResource } from "../../../_ui/common/nav/action_nav/resource"
import { ValidateBoardActionState } from "../../../../../ui/vendor/getto-application/board/action_validate_board/core/action"
import { AuthenticatePasswordCoreAction, AuthenticatePasswordCoreState } from "./core/action"
import { AuthenticatePasswordFormAction } from "./form/action"

export type AuthenticatePasswordView = ApplicationView<AuthenticatePasswordResource>

export type AuthenticatePasswordResource = SignLinkResource &
    Readonly<{ authenticate: AuthenticatePasswordAction }>

export interface AuthenticatePasswordAction extends ApplicationAction {
    readonly core: AuthenticatePasswordCoreAction
    readonly form: AuthenticatePasswordFormAction
}

export type AuthenticatePasswordResourceState = Readonly<{
    state: Readonly<{
        core: AuthenticatePasswordCoreState
        form: ValidateBoardActionState
    }>
}>
