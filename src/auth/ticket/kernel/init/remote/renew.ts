import { env } from "../../../../../y_environment/_ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { Clock } from "../../../../../z_details/_ui/clock/infra"
import { RenewAuthTicketRemote } from "../../infra"

import { convertAuthRemote } from "../../convert"

export function newRenewAuthTicketRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): RenewAuthTicketRemote {
    return async () => {
        const AuthenticateResponsePb = pb.auth.ticket.api.AuthenticateResponsePb

        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: convertAuthRemote(clock, { roles: ["admin", "dev-docs"] }),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/auth-ticket",
                method: "PATCH",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            return {
                success: true,
                value: convertAuthRemote(
                    clock,
                    decodeProtobuf(AuthenticateResponsePb, await response.text()),
                ),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
