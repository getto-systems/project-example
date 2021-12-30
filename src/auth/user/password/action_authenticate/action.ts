import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { SignLink } from "../../../sign/action_nav/resource"
import { InputLoginIDAction } from "../../login_id/input/action_input/action"
import { InputPasswordAction } from "../input/action_input/action"
import { ValidateBoardAction } from "../../../../../ui/vendor/getto-application/board/action_validate_board/action"

import { AuthenticatePasswordMethod } from "../authenticate/method"
import { GetScriptPathMethod } from "../../../sign/get_script_path/method"
import {
    SaveAuthProfileMethod,
    StartContinuousRenewMethod,
} from "../../../ticket/start_continuous_renew/method"

import { AuthenticatePasswordEvent } from "../authenticate/event"
import { StartContinuousRenewEvent } from "../../../ticket/start_continuous_renew/event"

import {
    LoadScriptError,
    ConvertScriptPathResult,
} from "../../../sign/get_script_path/data"

export interface AuthenticatePasswordAction
    extends ApplicationStateAction<AuthenticatePasswordState> {
    readonly link: SignLink

    readonly loginID: InputLoginIDAction
    readonly password: InputPasswordAction
    readonly validate: ValidateBoardAction

    clear(): AuthenticatePasswordState
    submit(): Promise<AuthenticatePasswordState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordState>
}

export const authenticatePasswordFieldNames = ["loginID", "password"] as const
export type AuthenticatePasswordFieldName = typeof authenticatePasswordFieldNames[number]

export type AuthenticatePasswordMaterial = Readonly<{
    save: SaveAuthProfileMethod
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
