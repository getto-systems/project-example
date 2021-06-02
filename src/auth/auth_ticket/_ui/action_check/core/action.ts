import { ApplicationStateAction } from "../../../../../../ui/vendor/getto-application/action/action"

import { GetScriptPathMethod } from "../../../../_ui/common/secure/get_script_path/method"
import {
    SaveAuthTicketMethod,
    StartContinuousRenewMethod,
} from "../../start_continuous_renew/method"
import { RenewAuthTicketMethod, CheckAuthTicketMethod } from "../../check/method"

import { StartContinuousRenewEvent } from "../../start_continuous_renew/event"
import { CheckAuthTicketEvent } from "../../check/event"

import {
    ConvertScriptPathResult,
    LoadScriptError,
} from "../../../../_ui/common/secure/get_script_path/data"

export interface CheckAuthTicketCoreAction extends ApplicationStateAction<CheckAuthTicketCoreState> {
    succeedToInstantLoad(): Promise<CheckAuthTicketCoreState>
    failedToInstantLoad(): Promise<CheckAuthTicketCoreState>
    loadError(err: LoadScriptError): Promise<CheckAuthTicketCoreState>
}

export type CheckAuthTicketCoreMaterial = Readonly<{
    renew: CheckAuthTicketMethod
    forceRenew: RenewAuthTicketMethod
    save: SaveAuthTicketMethod
    startContinuousRenew: StartContinuousRenewMethod
    getSecureScriptPath: GetScriptPathMethod
}>

export type CheckAuthTicketCoreState =
    | Readonly<{ type: "initial-check" }>
    | Exclude<CheckAuthTicketEvent, { type: "try-to-instant-load" | "succeed-to-renew" }>
    | StartContinuousRenewEvent
    | Readonly<{ type: "try-to-instant-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "try-to-load"; scriptPath: ConvertScriptPathResult }>
    | Readonly<{ type: "load-error"; err: LoadScriptError }>

export const initialCheckAuthTicketCoreState: CheckAuthTicketCoreState = {
    type: "initial-check",
}
