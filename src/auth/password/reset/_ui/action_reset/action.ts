import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { SignLink } from "../../../../_ui/common/nav/action_nav/resource"
import { InputLoginIDAction } from "../../../../login_id/_ui/action_input/action"
import { InputPasswordAction } from "../../../_ui/action_input/action"
import { ValidateBoardAction } from "../../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { ResetPasswordMethod } from "../reset/method"
import { GetScriptPathMethod } from "../../../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../../../../auth_ticket/_ui/start_continuous_renew/method"

import { ResetPasswordEvent } from "../reset/event"
import { StartContinuousRenewEvent } from "../../../../auth_ticket/_ui/start_continuous_renew/event"

import { ResetPasswordFields } from "../reset/data"
import {
    LoadScriptError,
    ConvertScriptPathResult,
} from "../../../../_ui/common/secure/get_script_path/data"

export interface ResetPasswordAction extends ApplicationStateAction<ResetPasswordState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateResetAction

    clear(): void
    submit(): Promise<ResetPasswordState>
    loadError(err: LoadScriptError): Promise<ResetPasswordState>
}

export const resetPasswordFields = ["loginID", "password"] as const
export type ValidateResetAction = ValidateBoardAction<
    typeof resetPasswordFields[number],
    ResetPasswordFields
>

export type ResetPasswordMaterial = Readonly<{
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
    reset: ResetPasswordMethod
}>

export type ResetPasswordState =
    | Readonly<{ type: "initial-reset" }>
    | Exclude<ResetPasswordEvent, { type: "succeed-to-reset" }>
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialResetPasswordState: ResetPasswordState = {
    type: "initial-reset",
}
