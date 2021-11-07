import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

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
