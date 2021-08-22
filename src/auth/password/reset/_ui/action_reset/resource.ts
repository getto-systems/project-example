import {
    ApplicationAction,
    ApplicationView,
} from "../../../../../../ui/vendor/getto-application/action/action"

import { SignLinkResource } from "../../../../_ui/common/nav/action_nav/resource"
import { ValidateBoardActionState } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"
import { ResetPasswordCoreAction, ResetPasswordCoreState } from "./core/action"
import { ResetPasswordFormAction } from "./form/action"

export type ResetPasswordView = ApplicationView<ResetPasswordResource>

export type ResetPasswordResource = SignLinkResource & Readonly<{ reset: ResetPasswordAction }>

export interface ResetPasswordAction extends ApplicationAction {
    readonly core: ResetPasswordCoreAction
    readonly form: ResetPasswordFormAction
}

export type ResetPasswordResourceState = Readonly<{
    state: Readonly<{
        core: ResetPasswordCoreState
        form: ValidateBoardActionState
    }>
}>
