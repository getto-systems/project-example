import { env } from "../../../../../../../y_environment/_ui/env"
import {
    ResetPasswordResult_pb,
    ResetPassword_pb,
} from "../../../../../../_ui/y_protobuf/api_pb.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { ResetPasswordRemote } from "../../infra"

import { convertAuthRemote } from "../../../../../../auth_ticket/_ui/kernel/converter"
import { Clock } from "../../../../../../../z_details/_ui/clock/infra"
import { ResetPasswordRemoteError } from "../../data"

export function newResetPasswordRemote(
    feature: RemoteOutsideFeature,
    clock: Clock,
): ResetPasswordRemote {
    return async (resetToken, fields) => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
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
                body: encodeProtobuf(ResetPassword_pb, (message) => {
                    message.resetToken = resetToken
                    message.loginId = fields.loginID
                    message.password = fields.password
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(ResetPasswordResult_pb, await response.text())
            if (!result.success) {
                return { success: false, err: mapError(result) }
            }
            return {
                success: true,
                value: convertAuthRemote(clock, { roles: result.value?.roles || [] }),
            }
        } catch (err) {
            return remoteInfraError(err)
        }

        function mapError(result: ResetPasswordResult_pb): ResetPasswordRemoteError {
            if (!result.err || !result.err.type) {
                return { type: "invalid-reset" }
            }
            switch (result.err.type) {
                case ResetPasswordResult_pb.ErrorType.INVALID_RESET:
                    return { type: "invalid-reset" }

                case ResetPasswordResult_pb.ErrorType.ALREADY_RESET:
                    return { type: "already-reset" }
            }
        }
    }
}
