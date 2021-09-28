import { DelayTime } from "../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../z_details/_ui/remote/infra"

import { CheckDeployExistsRemoteError } from "./data"

export type FindNextVersionInfra = Readonly<{
    version: string
    versionSuffix: string
    check: CheckDeployExistsRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface CheckDeployExistsRemote {
    (url: CheckDeployExistsURL): Promise<CheckDeployExistsRemoteResult>
}
export type CheckDeployExistsRemoteResult = RemoteResult<
    CheckDeployExistsResponse,
    CheckDeployExistsRemoteError
>

export type CheckDeployExistsURL = string
export type CheckDeployExistsResponse = Readonly<{ found: boolean }>
