import { RemoteTypes } from "../../../../../z_details/_ui/remote/infra"
import { DelayTime } from "../../../../../z_details/_ui/config/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"
import { ResetSessionID } from "../data"

export type RequestResetTokenInfra = Readonly<{
    requestToken: RequestResetTokenRemotePod
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

type RequestResetTokenRemoteTypes = RemoteTypes<
    RequestResetTokenFields,
    ResetSessionID,
    string,
    RequestResetTokenRemoteError
>
export type RequestResetTokenRemotePod = RequestResetTokenRemoteTypes["pod"]
export type RequestResetTokenResult = RequestResetTokenRemoteTypes["result"]
export type RequestResetTokenSimulator = RequestResetTokenRemoteTypes["simulator"]
