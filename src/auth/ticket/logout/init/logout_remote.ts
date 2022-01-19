import { env } from "../../../../y_environment/ui/env"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../z_lib/ui/remote/init/helper"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { LogoutRemote } from "../infra"
import { RemoteCommonError } from "../../../../z_lib/ui/remote/data"
import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

export function newLogoutRemote(feature: RemoteOutsideFeature): LogoutRemote {
    return async () => fetchRemote(feature)
}
async function fetchRemote(
    feature: RemoteOutsideFeature,
): Promise<RemoteResult<true, RemoteCommonError>> {
    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/ticket",
            method: "DELETE",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
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
