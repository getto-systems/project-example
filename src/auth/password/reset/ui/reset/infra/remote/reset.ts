import { env } from "../../../../../../../y_environment/_ui/env"
import {
    ResetPasswordResult_pb,
    ResetPassword_pb,
} from "../../../../../../_ui/y_protobuf/api_pb.js"

import {
    convertRemote,
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { ResetPasswordRemotePod } from "../../infra"

import {
    ApiAuthenticateResponse,
    ApiCommonError,
    ApiResult,
} from "../../../../../../../z_details/_ui/api/data"

export function newResetPasswordRemote(feature: RemoteOutsideFeature): ResetPasswordRemotePod {
    type ResetParams = Readonly<{
        resetToken: string
        fields: Readonly<{
            loginID: string
            password: string
        }>
    }>
    type ResetResult = ApiResult<ApiAuthenticateResponse, ApiCommonError | ResetError>
    type ResetError = Readonly<{ type: "invalid-reset" }> | Readonly<{ type: "already-reset" }>

    return convertRemote(async (params: ResetParams): Promise<ResetResult> => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: { roles: ["admin", "dev-docs"] } }
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
                    message.resetToken = params.resetToken
                    message.loginId = params.fields.loginID
                    message.password = params.fields.password
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
                value: {
                    roles: result.value?.roles || [],
                },
            }
        } catch (err) {
            return remoteInfraError(err)
        }

        function mapError(result: ResetPasswordResult_pb): ResetError {
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
    })
}
