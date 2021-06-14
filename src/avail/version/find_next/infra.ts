import { RemoteTypes } from "../../../z_details/_ui/remote/infra"
import { DelayTime } from "../../../z_details/_ui/config/infra"

import { CheckDeployExistsRemoteError } from "./data"

export type FindNextVersionInfra = Readonly<{
    version: string
    versionSuffix: string
    check: CheckDeployExistsRemotePod
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

type CheckDeployExistsRemoteTypes = RemoteTypes<
    CheckDeployExistsURL,
    CheckDeployExistsResponse,
    CheckDeployExistsResponse,
    CheckDeployExistsRemoteError
>
export type CheckDeployExistsRemotePod = CheckDeployExistsRemoteTypes["pod"]
export type CheckDeployExistsRemote = CheckDeployExistsRemoteTypes["remote"]

export type CheckDeployExistsURL = string
export type CheckDeployExistsResponse = Readonly<{ found: boolean }>
