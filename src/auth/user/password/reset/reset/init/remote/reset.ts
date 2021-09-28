import { env } from "../../../../../../../y_environment/ui/env"
import pb from "../../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_lib/ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_lib/ui/remote/feature"

import { Clock } from "../../../../../../../z_lib/ui/clock/infra"
import { ResetPasswordRemote } from "../../infra"

import { convertAuthRemote } from "../../../../../../ticket/kernel/convert"

export function newResetPasswordRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): ResetPasswordRemote {
    return async (resetToken, fields) => {
        const ResetPasswordPb = pb.auth.user.password.reset.api.ResetPasswordPb
        const ResetPasswordResultPb = pb.auth.user.password.reset.api.ResetPasswordResultPb
        const ResetPasswordErrorKind = pb.auth.user.password.reset.api.ResetPasswordErrorKindPb

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
                path: "/auth/password/reset",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(ResetPasswordPb, (message) => {
                    message.resetToken = resetToken
                    message.loginId = fields.loginID
                    message.password = fields.password
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(ResetPasswordResultPb, await response.text())
            if (!result.success) {
                if (!result.err) {
                    return { success: false, err: { type: "invalid-reset" } }
                }
                switch (result.err.kind) {
                    case ResetPasswordErrorKind.INVALID_RESET:
                        return { success: false, err: { type: "invalid-reset" } }

                    case ResetPasswordErrorKind.ALREADY_RESET:
                        return { success: false, err: { type: "already-reset" } }
                }
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
