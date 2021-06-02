import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { AuthenticatePasswordMethod } from "../../authenticate/method"
import { GetScriptPathMethod } from "../../../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../../../../auth_ticket/_ui/start_continuous_renew/method"

import { AuthenticatePasswordEvent } from "../../authenticate/event"
import { StartContinuousRenewEvent } from "../../../../auth_ticket/_ui/start_continuous_renew/event"

import {
    LoadScriptError,
    ConvertScriptPathResult,
} from "../../../../_ui/common/secure/get_script_path/data"
import { AuthenticatePasswordFields } from "../../authenticate/data"
import { ConvertBoardResult } from "../../../../../../ui/vendor/getto-application/board/kernel/data"

export interface AuthenticatePasswordCoreAction
    extends ApplicationStateAction<AuthenticatePasswordCoreState> {
    submit(
        fields: ConvertBoardResult<AuthenticatePasswordFields>,
    ): Promise<AuthenticatePasswordCoreState>
    loadError(err: LoadScriptError): Promise<AuthenticatePasswordCoreState>
}

export type AuthenticatePasswordCoreMaterial = Readonly<{
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
    authenticate: AuthenticatePasswordMethod
}>

export type AuthenticatePasswordCoreState =
    | Readonly<{ type: "initial-login" }>
    | Exclude<AuthenticatePasswordEvent, { type: "succeed-to-login" }>
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialAuthenticatePasswordCoreState: AuthenticatePasswordCoreState = {
    type: "initial-login",
}
