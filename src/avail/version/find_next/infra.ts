import { RemoteResult } from "../../../z_lib/ui/remote/infra"

import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { ApplicationTargetPath, CheckDeployExistsRemoteError } from "./data"

export interface ApplicationTargetPathDetecter {
    (): ConvertLocationResult<ApplicationTargetPath>
}

export interface CheckDeployExistsRemote {
    (url: CheckDeployExistsURL): Promise<CheckDeployExistsRemoteResult>
}
export type CheckDeployExistsRemoteResult = RemoteResult<
    CheckDeployExistsResponse,
    CheckDeployExistsRemoteError
>

export type CheckDeployExistsURL = string
export type CheckDeployExistsResponse = Readonly<{ found: boolean }>
