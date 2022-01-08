import { remoteInfraError, remoteServerError } from "../../../../z_lib/ui/remote/helper"

import { CheckDeployExistsRemote } from "../infra"

export function newCheckDeployExistsRemote(): CheckDeployExistsRemote {
    return async (url) => {
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
    }
}
