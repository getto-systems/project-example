import { env } from "../../../../../y_environment/_ui/env"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../ui/vendor/getto-application/infra/remote/helper"
import { apiInfraError, apiRequest, apiStatusError } from "../../../../../z_details/_ui/api/helper"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/infra"
import { ClearAuthTicketRemotePod } from "../infra"

export function newClearAuthTicketRemote(feature: RemoteOutsideFeature): ClearAuthTicketRemotePod {
    return convertRemote(async () => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: true }
            }

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/auth/clear",
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
    })
}
