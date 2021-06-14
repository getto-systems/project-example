import {
    convertRemote,
    remoteInfraError,
    remoteServerError,
} from "../../../../../z_details/_ui/remote/helper"

import { CheckDeployExistsRemotePod } from "../../infra"

import { ApiInfraError, ApiResult, ApiServerError } from "../../../../../z_details/_ui/api/data"

export function newCheckDeployExistsRemote(): CheckDeployExistsRemotePod {
    type CheckResult = ApiResult<CheckResponse, CheckError>
    type CheckResponse = Readonly<{ found: boolean }>
    type CheckError = ApiServerError | ApiInfraError

    return convertRemote(async (url: string): Promise<CheckResult> => {
        try {
            const response = await fetch(url, { method: "HEAD" })
            if (!response.ok) {
                if (response.status >= 500) {
                    return remoteServerError()
                }
                return { success: true, value: { found: false } }
            }
            return { success: true, value: { found: true } }
        } catch (err) {
            return remoteInfraError(err)
        }
    })
}
