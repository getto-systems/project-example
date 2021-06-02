import { env } from "../../../../../../../y_environment/env"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../../../ui/vendor/getto-application/infra/remote/helper"
import {
    apiInfraError,
    apiRequest,
    apiStatusError,
} from "../../../../../../../z_details/_ui/api/helper"

import { RemoteOutsideFeature } from "../../../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { SendResetTokenRemotePod } from "../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../../z_details/_ui/api/data"

export function newSendResetTokenRemote(feature: RemoteOutsideFeature): SendResetTokenRemotePod {
    type SendTokenResult = ApiResult<true, ApiCommonError>

    return convertRemote(
        async (): Promise<SendTokenResult> => {
            try {
                const mock = true
                if (mock) {
                    // TODO api の実装が終わったらつなぐ
                    return { success: true, value: true }
                }

                const request = apiRequest(
                    remoteFeature(env.apiServerURL, feature),
                    "/auth/password/reset/token/sender",
                    "POST",
                )
                const response = await fetch(request.url, request.options)

                if (!response.ok) {
                    return apiStatusError(response.status)
                }
                return { success: true, value: true }
            } catch (err) {
                return apiInfraError(err)
            }
        },
    )
}
