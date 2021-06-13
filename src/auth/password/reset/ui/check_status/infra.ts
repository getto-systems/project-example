import { RemoteTypes } from "../../../../../z_details/_ui/remote/infra"
import { Limit, WaitTime } from "../../../../../z_details/_ui/config/infra"

import { CheckResetTokenSendingStatusRemoteError, ResetTokenSendingResult } from "./data"
import { ResetSessionID } from "../data"

export type CheckResetTokenSendingStatusInfra = Readonly<{
    sendToken: SendResetTokenRemotePod
    getStatus: GetResetTokenSendingStatusRemotePod
    config: Readonly<{
        wait: WaitTime
        limit: Limit
    }>
}>

type SendRemoteTypes = RemoteTypes<null, true, true, CheckResetTokenSendingStatusRemoteError>
export type SendResetTokenRemotePod = SendRemoteTypes["pod"]
export type SendResetTokenResult = SendRemoteTypes["result"]
export type SendResetTokenSimulator = SendRemoteTypes["simulator"]

type GetSendingStatusRemoteTypes = RemoteTypes<
    ResetSessionID,
    ResetTokenSendingResult,
    ResetTokenSendingResult,
    CheckResetTokenSendingStatusRemoteError
>
export type GetResetTokenSendingStatusRemotePod = GetSendingStatusRemoteTypes["pod"]
export type GetResetTokenSendingStatusResult = GetSendingStatusRemoteTypes["result"]
export type GetResetTokenSendingStatusSimulator = GetSendingStatusRemoteTypes["simulator"]
