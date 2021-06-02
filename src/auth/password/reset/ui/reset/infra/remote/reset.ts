import { env } from "../../../../../../../y_environment/env"
import { ResetPasswordResult_pb, ResetPassword_pb } from "../../../../../../_ui/y_protobuf/api_pb.js"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../../../ui/vendor/getto-application/infra/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"
import {
    apiInfraError,
    apiRequest,
    apiStatusError,
} from "../../../../../../../z_details/_ui/api/helper"

import { RemoteOutsideFeature } from "../../../../../../../../ui/vendor/getto-application/infra/remote/infra"
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

    return convertRemote(
        async (params: ResetParams): Promise<ResetResult> => {
            try {
                const mock = true
                if (mock) {
                    // TODO api の実装が終わったらつなぐ
                    return { success: true, value: { roles: ["admin", "dev-docs"] } }
                }

                const request = apiRequest(
                    remoteFeature(env.apiServerURL, feature),
                    "/auth/password/reset",
                    "POST",
                )
                const response = await fetch(request.url, {
                    ...request.options,
                    body: encodeProtobuf(ResetPassword_pb, (message) => {
                        message.resetToken = params.resetToken
                        message.loginId = params.fields.loginID
                        message.password = params.fields.password
                    }),
                })

                if (!response.ok) {
                    return apiStatusError(response.status)
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
                return apiInfraError(err)
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
        },
    )
}
