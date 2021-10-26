import { DelayTime } from "../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../z_lib/ui/remote/infra"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { ApplicationTargetPath, CheckDeployExistsRemoteError } from "./data"

export interface FindNextVersionDetecter {
    (): ConvertLocationResult<ApplicationTargetPath>
}

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
