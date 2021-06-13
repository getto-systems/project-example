import { env } from "../../../../../../y_environment/_ui/env"
import { AuthenticateResponse_pb } from "../../../../../_ui/y_protobuf/api_pb.js"

import { remoteFeature, convertRemote } from "../../../../../../z_details/_ui/remote/helper"
import {
    apiInfraError,
    apiRequest,
    apiStatusError,
} from "../../../../../../z_details/_ui/api/helper"
import { decodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { RenewAuthTicketRemotePod } from "../../infra"

export function newRenewAuthTicketRemote(feature: RemoteOutsideFeature): RenewAuthTicketRemotePod {
    return convertRemote(async () => {
        try {
            const mock = false
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: { roles: ["admin", "dev-docs"] } }
            }

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/auth/auth-ticket/renew",
                "POST",
            )
            const response = await fetch(request.url, request.options)

            if (!response.ok) {
                return apiStatusError(response.status)
            }

            return {
                success: true,
                value: decodeProtobuf(AuthenticateResponse_pb, await response.text()),
            }
        } catch (err) {
            return apiInfraError(err)
        }
    })
}
