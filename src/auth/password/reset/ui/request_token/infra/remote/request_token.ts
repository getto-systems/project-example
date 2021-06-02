import { env } from "../../../../../../../y_environment/_ui/env"
import {
    RequestResetTokenResult_pb,
    RequestResetToken_pb,
} from "../../../../../../_ui/y_protobuf/api_pb.js"

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
import { RequestResetTokenRemotePod } from "../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../../z_details/_ui/api/data"

export function newRequestResetTokenRemote(
    feature: RemoteOutsideFeature,
): RequestResetTokenRemotePod {
    type RequestTokenFields = Readonly<{
        loginID: string
    }>
    type RequestTokenResult = ApiResult<string, ApiCommonError | RequestTokenError>
    type RequestTokenError = Readonly<{ type: "invalid-reset" }>

    return convertRemote(async (fields: RequestTokenFields): Promise<RequestTokenResult> => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: "reset-session-id" }
            }

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/auth/password/reset/token",
                "POST",
            )
            const response = await fetch(request.url, {
                ...request.options,
                body: encodeProtobuf(RequestResetToken_pb, (message) => {
                    message.loginId = fields.loginID
                }),
            })

            if (!response.ok) {
                return apiStatusError(response.status)
            }

            const result = decodeProtobuf(RequestResetTokenResult_pb, await response.text())
            if (!result.success) {
                return { success: false, err: mapError(result) }
            }
            return {
                success: true,
                value: result.value?.sessionId || "",
            }
        } catch (err) {
            return apiInfraError(err)
        }

        function mapError(_result: RequestResetTokenResult_pb): RequestTokenError {
            return { type: "invalid-reset" }
        }
    })
}
