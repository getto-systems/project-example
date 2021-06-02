import { RemoteTypes } from "../../../../ui/vendor/getto-application/infra/remote/infra"
import { DelayTime } from "../../../../ui/vendor/getto-application/infra/config/infra"

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
export type CheckDeployExistsRemoteResult = CheckDeployExistsRemoteTypes["result"]
export type CheckDeployExistsSimulator = CheckDeployExistsRemoteTypes["simulator"]

export type CheckDeployExistsURL = string
export type CheckDeployExistsResponse = Readonly<{ found: boolean }>
