import { env } from "../../../../../../../y_environment/_ui/env"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { SendResetTokenRemote } from "../../infra"

export function newSendResetTokenRemote(feature: RemoteOutsideFeature): SendResetTokenRemote {
    return async () => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/password/reset/token/sender",
                method: "POST",
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