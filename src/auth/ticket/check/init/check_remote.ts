import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf } from "../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { Clock } from "../../../../z_lib/ui/clock/infra"
import { CheckAuthTicketRemote } from "../infra"

import { convertCheckRemote } from "../convert"

export function newCheckAuthTicketRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): CheckAuthTicketRemote {
    return async () => {
        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: convertCheckRemote(clock, ["admin", "dev-docs"]),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/ticket",
                method: "PATCH",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.ticket.check.service.CheckAuthTicketMaskedResponsePb,
                await response.text(),
            )

            return {
                success: true,
                value: convertCheckRemote(clock, message.roles?.grantedRoles || []),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
