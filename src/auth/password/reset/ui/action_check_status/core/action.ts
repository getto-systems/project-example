import { ApplicationStateAction } from "../../../../../../../ui/vendor/getto-application/action/action"
import { CheckResetTokenSendingStatusEvent } from "../../check_status/event"
import { CheckResetTokenSendingStatusPod, CheckSendingStatusMethod } from "../../check_status/method"

export type CheckResetTokenSendingStatusCoreAction = ApplicationStateAction<
    CheckResetTokenSendingStatusCoreState
>

export type CheckResetTokenSendingStatusCoreMaterial = Readonly<{
    checkStatus: CheckSendingStatusMethod
}>
export type CheckResetTokenSendingStatusCoreMaterialPod = Readonly<{
    initCheckStatus: CheckResetTokenSendingStatusPod
}>

export type CheckResetTokenSendingStatusCoreState =
    | Readonly<{ type: "initial-check-status" }>
    | CheckResetTokenSendingStatusEvent

export const initialCheckResetTokenSendingStatusCoreState: CheckResetTokenSendingStatusCoreState = {
    type: "initial-check-status",
}
