import { DelayTime } from "../../../../../z_details/_ui/config/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"
import { ResetSessionID } from "../data"
import { RemoteResult } from "../../../../../z_details/_ui/remote/data"

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
