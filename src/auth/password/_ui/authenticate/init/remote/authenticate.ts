import { env } from "../../../../../../y_environment/_ui/env"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { Clock } from "../../../../../../z_details/_ui/clock/infra"
import { AuthenticatePasswordRemote } from "../../infra"

import { convertAuthRemote } from "../../../../../auth_ticket/_ui/kernel/convert"

export function newAuthenticatePasswordRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): AuthenticatePasswordRemote {
    return async (fields) => {
        const pb = await import(/* webpackMode: "eager" */ "../../../y_protobuf/api_pb.js")
        const AuthenticatePasswordPb = pb.auth.password.api.AuthenticatePasswordPb
        const AuthenticatePasswordResultPb = pb.auth.password.api.AuthenticatePasswordResultPb

        try {
            const mock = false
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return {
                    success: true,
                    value: convertAuthRemote(clock, { roles: ["admin", "dev-docs"] }),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/password/authenticate",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(AuthenticatePasswordPb, (message) => {
                    message.loginId = fields.loginID
                    message.password = fields.password
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(AuthenticatePasswordResultPb, await response.text())
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
