import { WorkerProxySpec } from "../../../../../../../../ui/vendor/getto-application/action/worker/message"

import { CheckResetTokenSendingStatusEvent } from "../../../check_status/event"

import { ConvertLocationResult } from "../../../../../../../../ui/vendor/getto-application/location/data"
import { ResetSessionID } from "../../../data"

export type CheckPasswordResetSendingStatusProxyMaterial = Readonly<{
    checkStatus: CheckStatus["method"]
}>
export type CheckPasswordResetSendingStatusProxyMessage = CheckStatus["message"]
export type CheckPasswordResetSendingStatusProxyResponse = CheckStatus["response"]

type CheckStatus = WorkerProxySpec<
    "checkStatus",
    ConvertLocationResult<ResetSessionID>,
    CheckResetTokenSendingStatusEvent
>
