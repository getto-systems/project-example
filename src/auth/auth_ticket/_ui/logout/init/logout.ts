import { env } from "../../../../../y_environment/_ui/env"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_details/_ui/remote/helper"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { LogoutRemote } from "../infra"

export function newLogoutRemote(feature: RemoteOutsideFeature): LogoutRemote {
    return async () => {
        try {
            const mock = false
            if (mock) {
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/auth-ticket",
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
}
