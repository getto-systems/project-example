import { env } from "../../../../../../../y_environment/_ui/env"

import {
    convertRemote,
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { SendResetTokenRemotePod } from "../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../../z_details/_ui/api/data"

export function newSendResetTokenRemote(feature: RemoteOutsideFeature): SendResetTokenRemotePod {
    type SendTokenResult = ApiResult<true, ApiCommonError>

    return convertRemote(async (): Promise<SendTokenResult> => {
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
    })
}
