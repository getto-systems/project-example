import { env } from "../../../../y_environment/ui/env"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../common/util/remote/init/helper"

import { LogoutRemote } from "../infra"
import { RemoteCommonError } from "../../../../common/util/remote/data"
import { RemoteResult } from "../../../../common/util/remote/infra"

export function newLogoutRemote(): LogoutRemote {
    return async () => fetchRemote()
}
async function fetchRemote(): Promise<RemoteResult<true, RemoteCommonError>> {
    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/ticket",
            method: "DELETE",
        })
        const response = await fetch(opts.url, opts.options)

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}
