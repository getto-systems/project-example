import { ApplicationStateAction } from "../../../../ui/vendor/getto-application/action/action"

import { GetScriptPathMethod } from "../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../start_continuous_renew/method"
import { RenewAuthTicketMethod, CheckAuthTicketMethod } from "../check/method"

import { StartContinuousRenewEvent } from "../start_continuous_renew/event"
import { CheckAuthTicketEvent } from "../check/event"

import {
    ConvertScriptPathResult,
    LoadScriptError,
} from "../../_ui/common/secure/get_script_path/data"

export interface CheckAuthTicketAction extends ApplicationStateAction<CheckAuthTicketState> {
    succeedToInstantLoad(): Promise<CheckAuthTicketState>
    failedToInstantLoad(): Promise<CheckAuthTicketState>
    loadError(err: LoadScriptError): Promise<CheckAuthTicketState>
}

export type CheckAuthTicketMaterial = Readonly<{
    renew: CheckAuthTicketMethod
    forceRenew: RenewAuthTicketMethod
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
}>

export type CheckAuthTicketState =
    | Readonly<{ type: "initial-check" }>
    | Exclude<CheckAuthTicketEvent, { type: "try-to-instant-load" | "succeed-to-renew" }>
    | StartContinuousRenewEvent
    | Readonly<{ type: "try-to-instant-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialCheckAuthTicketState: CheckAuthTicketState = {
    type: "initial-check",
}
