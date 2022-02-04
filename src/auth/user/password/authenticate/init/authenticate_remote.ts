import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { Clock } from "../../../../../z_lib/ui/clock/infra"
import { AuthenticatePasswordRemote } from "../infra"

import { convertCheckRemote } from "../../../../ticket/check/convert"

export function newAuthenticatePasswordRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): AuthenticatePasswordRemote {
    return async (fields) => {
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
                path: "/auth/user/password/authenticate",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.auth.user.password.service.AuthenticatePasswordRequestPb,
                    (message) => {
                        message.loginId = fields.loginID
                        message.password = fields.password
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.user.password.service.AuthenticatePasswordResponsePb,
                await response.text(),
            )
            if (!message.success) {
                return { success: false, err: { type: "invalid-password" } }
            }
            return {
                success: true,
                value: convertCheckRemote(clock, message.roles?.grantedRoles || []),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
