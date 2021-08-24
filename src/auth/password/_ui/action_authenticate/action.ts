import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { SignLink } from "../../../_ui/common/nav/action_nav/resource"
import { InputLoginIDAction } from "../../../login_id/_ui/action_input/action"
import { InputPasswordAction } from "../action_input/action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { AuthenticatePasswordMethod } from "../authenticate/method"
import { GetScriptPathMethod } from "../../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../../../auth_ticket/_ui/start_continuous_renew/method"

import { AuthenticatePasswordEvent } from "../authenticate/event"
import { StartContinuousRenewEvent } from "../../../auth_ticket/_ui/start_continuous_renew/event"

import {
    LoadScriptError,
    ConvertScriptPathResult,
} from "../../../_ui/common/secure/get_script_path/data"

export interface AuthenticatePasswordAction
    extends ApplicationStateAction<AuthenticatePasswordState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): void
    submit(): Promise<AuthenticatePasswordState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordState>
}

export const authenticatePasswordFieldNames = ["loginID", "password"] as const
export type AuthenticatePasswordFieldName = typeof authenticatePasswordFieldNames[number]

export type AuthenticatePasswordMaterial = Readonly<{
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
    authenticate: AuthenticatePasswordMethod
}>

export type AuthenticatePasswordState =
    | Readonly<{ type: "initial-login" }>
    | Exclude<AuthenticatePasswordEvent, { type: "succeed-to-login" }>
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialAuthenticatePasswordState: AuthenticatePasswordState = {
    type: "initial-login",
}
