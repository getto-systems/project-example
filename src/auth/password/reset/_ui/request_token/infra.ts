import { DelayTime } from "../../../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../../../z_details/_ui/remote/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"
import { ResetSessionID } from "../data"

export type RequestResetTokenInfra = Readonly<{
    requestToken: RequestResetTokenRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface RequestResetTokenRemote {
    (fields: RequestResetTokenFields): Promise<RequestResetTokenRemoteResult>
}
export type RequestResetTokenRemoteResult = RemoteResult<
    ResetSessionID,
    RequestResetTokenRemoteError
>
