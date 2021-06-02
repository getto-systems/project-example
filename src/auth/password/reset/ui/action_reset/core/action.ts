import { ApplicationStateAction } from "../../../../../../../ui/vendor/getto-application/action/action"

import { ResetPasswordMethod } from "../../reset/method"
import { GetScriptPathMethod } from "../../../../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../../../../../auth_ticket/_ui/start_continuous_renew/method"

import { ResetPasswordEvent } from "../../reset/event"
import { StartContinuousRenewEvent } from "../../../../../auth_ticket/_ui/start_continuous_renew/event"

import { ResetPasswordFields } from "../../reset/data"
import {
    LoadScriptError,
    ConvertScriptPathResult,
} from "../../../../../_ui/common/secure/get_script_path/data"
import { ConvertBoardResult } from "../../../../../../../ui/vendor/getto-application/board/kernel/data"

export interface ResetPasswordCoreAction extends ApplicationStateAction<ResetPasswordCoreState> {
    submit(fields: ConvertBoardResult<ResetPasswordFields>): Promise<ResetPasswordCoreState>
    loadError(err: LoadScriptError): Promise<ResetPasswordCoreState>
}

export type ResetPasswordCoreMaterial = Readonly<{
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
    reset: ResetPasswordMethod
}>

export type ResetPasswordCoreState =
    | Readonly<{ type: "initial-reset" }>
    | Exclude<ResetPasswordEvent, { type: "succeed-to-reset" }>
    | Exclude<StartContinuousRenewEvent, { type: "succeed-to-start-continuous-renew" }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialResetPasswordCoreState: ResetPasswordCoreState = {
    type: "initial-reset",
}
