import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../../z_lib/ui/remote/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"

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
    true,
    RequestResetTokenRemoteError
>
