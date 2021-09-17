import { DelayTime } from "../../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../../z_details/_ui/remote/infra"

import { ChangePasswordFields, ChangePasswordRemoteError } from "./data"

export type ChangePasswordInfra = Readonly<{
    change: ChangePasswordRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface ChangePasswordRemote {
    (fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult>
}
export type ChangePasswordRemoteResult = RemoteResult<true, ChangePasswordRemoteError>
