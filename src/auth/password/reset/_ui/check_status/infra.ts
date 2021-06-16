import { Limit, WaitTime } from "../../../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../../../z_details/_ui/remote/infra"

import { CheckResetTokenSendingStatusRemoteError, ResetTokenSendingResult } from "./data"
import { ResetSessionID } from "../data"

export type CheckResetTokenSendingStatusInfra = Readonly<{
    sendToken: SendResetTokenRemote
    getStatus: GetResetTokenSendingStatusRemote
    config: Readonly<{
        wait: WaitTime
        limit: Limit
    }>
}>

export interface SendResetTokenRemote {
    (): Promise<SendResetTokenRemoteResult>
}
export type SendResetTokenRemoteResult = RemoteResult<true, CheckResetTokenSendingStatusRemoteError>

export interface GetResetTokenSendingStatusRemote {
    (resetSessionID: ResetSessionID): Promise<GetResetTokenSendingStatusRemoteResult>
}
export type GetResetTokenSendingStatusRemoteResult = RemoteResult<
    ResetTokenSendingResult,
    CheckResetTokenSendingStatusRemoteError
>
