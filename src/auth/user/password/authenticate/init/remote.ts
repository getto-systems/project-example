import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { Clock } from "../../../../../z_lib/ui/clock/infra"
import { AuthenticatePasswordRemote } from "../infra"

import { convertAuthRemote } from "../../../../ticket/kernel/convert"

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
                    value: convertAuthRemote(clock, { roles: ["admin", "dev-docs"] }),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                // TODO /auth/user/password/authenticate にしたい
                path: "/auth/password/authenticate",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.auth.user.password.api.AuthenticatePasswordApiRequestPb,
                    (message) => {
                        message.loginId = fields.loginID
                        message.password = fields.password
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(
                pb.auth.user.password.api.AuthenticatePasswordApiResponsePb,
                await response.text(),
            )
            if (!result.success) {
                return { success: false, err: { type: "invalid-password" } }
            }
            return {
                success: true,
                value: convertAuthRemote(clock, { roles: result.value?.roles || [] }),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
