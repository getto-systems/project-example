import { LocationTypes } from "../../../../../../ui/vendor/getto-application/location/infra"

import { CheckResetTokenSendingStatusEvent } from "./event"

import { ResetSessionID } from "../data"

export interface CheckResetTokenSendingStatusPod {
    (detecter: CheckResetTokenSendingStatusLocationDetecter): CheckSendingStatusMethod
}

type CheckStatusLocationTypes = LocationTypes<ResetSessionID>
export type CheckResetTokenSendingStatusLocationDetecter = CheckStatusLocationTypes["detecter"]
export type CheckResetTokenSendingStatusLocationDetectMethod = CheckStatusLocationTypes["method"]
export type CheckResetTokenSendingStatusLocationInfo = CheckStatusLocationTypes["info"]

export interface CheckSendingStatusMethod {
    <S>(post: Post<CheckResetTokenSendingStatusEvent, S>): Promise<S>
}

interface Post<E, S> {
    (event: E): S
}
