import { env } from "../../../../../../../y_environment/_ui/env"
import {
    GetResetTokenSendingStatusResult_pb,
    GetResetTokenSendingStatus_pb,
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
import { GetResetTokenSendingStatusRemotePod } from "../../infra"

import { ApiCommonError, ApiResult } from "../../../../../../../z_details/_ui/api/data"

export function newGetResetTokenSendingStatusRemote(
    feature: RemoteOutsideFeature,
): GetResetTokenSendingStatusRemotePod {
    type GetSendingStatusResult = ApiResult<
        SendingTokenResult,
        ApiCommonError | GetSendingStatusError
    >
    type SendingTokenResult =
        | Readonly<{ done: false; status: Readonly<{ sending: boolean }> }>
        | Readonly<{ done: true; send: false; err: SendingTokenError }>
        | Readonly<{ done: true; send: true }>

    type SendingTokenError = "failed-to-connect-message-service"

    type GetSendingStatusError =
        | Readonly<{ type: "invalid-reset" }>
        | Readonly<{ type: "already-reset" }>

    return convertRemote(async (sessionID: string): Promise<GetSendingStatusResult> => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: { done: true, send: true } }
            }

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/auth/password/reset/status",
                "GET",
            )
            const response = await fetch(request.url, {
                ...request.options,
                body: encodeProtobuf(GetResetTokenSendingStatus_pb, (message) => {
                    message.sessionId = sessionID
                }),
            })

            if (!response.ok) {
                return apiStatusError(response.status)
            }

            const result = decodeProtobuf(
                GetResetTokenSendingStatusResult_pb,
                await response.text(),
            )
            if (!result.success) {
                return { success: false, err: mapError(result) }
            }
            return {
                success: true,
                value: toSendTokenResult(result),
            }
        } catch (err) {
            return apiInfraError(err)
        }

        function toSendTokenResult(
            result: GetResetTokenSendingStatusResult_pb,
        ): SendingTokenResult {
            if (!result.value) {
                return { done: false, status: { sending: false } }
            }
            const value = result.value
            if (!value.done) {
                return { done: false, status: { sending: value.sendingStatus || false } }
            }
            if (!value.send) {
                return { done: true, send: false, err: "failed-to-connect-message-service" }
            }
            return { done: true, send: true }
        }
        function mapError(result: GetResetTokenSendingStatusResult_pb): GetSendingStatusError {
            if (!result.err || !result.err.type) {
                return { type: "invalid-reset" }
            }
            switch (result.err.type) {
                case GetResetTokenSendingStatusResult_pb.ErrorType.INVALID_RESET:
                    return { type: "invalid-reset" }

                case GetResetTokenSendingStatusResult_pb.ErrorType.ALREADY_RESET:
                    return { type: "already-reset" }
            }
        }
    })
}
