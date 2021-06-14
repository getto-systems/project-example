import { env } from "../../../../../../y_environment/_ui/env"
import { AuthenticateResponse_pb } from "../../../../../_ui/y_protobuf/api_pb.js"

import {
    convertRemote,
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_details/_ui/remote/helper"
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

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/auth-ticket/renew",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            return {
                success: true,
                value: decodeProtobuf(AuthenticateResponse_pb, await response.text()),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    })
}
