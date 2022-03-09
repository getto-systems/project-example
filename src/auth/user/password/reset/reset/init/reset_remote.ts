import { env } from "../../../../../../y_environment/ui/env"
import pb from "../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { Clock } from "../../../../../../z_lib/ui/clock/infra"
import { ResetPasswordRemote } from "../infra"

import { convertCheckRemote } from "../../../../../ticket/check/convert"

export function newResetPasswordRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): ResetPasswordRemote {
    return async (resetToken, fields) => {
        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: convertCheckRemote(clock, []),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/user/password/reset",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.auth.user.password.reset.reset.service.ResetPasswordRequestPb,
                    (message) => {
                        message.resetToken = resetToken
                        message.loginId = fields.loginId
                        message.password = fields.password
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.user.password.reset.reset.service.ResetPasswordMaskedResponsePb,
                await response.text(),
            )
            if (!message.success) {
                switch (message.err) {
                    case pb.auth.user.password.reset.reset.service.ResetPasswordErrorKindPb
                        .INVALID_RESET:
                        return { success: false, err: { type: "invalid-reset" } }

                    case pb.auth.user.password.reset.reset.service.ResetPasswordErrorKindPb
                        .ALREADY_RESET:
                        return { success: false, err: { type: "already-reset" } }

                    default:
                        return { success: false, err: { type: "invalid-reset" } }
                }
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
