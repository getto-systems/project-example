import { env } from "../../../../../../../y_environment/_ui/env"
import {
    GetResetTokenSendingStatusResult_pb,
    GetResetTokenSendingStatus_pb,
} from "../../../../../../_ui/y_protobuf/api_pb.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { GetResetTokenSendingStatusRemote } from "../../infra"

import { CheckResetTokenSendingStatusRemoteError, ResetTokenSendingResult } from "../../data"

export function newGetResetTokenSendingStatusRemote(
    feature: RemoteOutsideFeature,
): GetResetTokenSendingStatusRemote {
    return async (sessionID) => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: { done: true, send: true } }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/password/reset/status",
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(GetResetTokenSendingStatus_pb, (message) => {
                    message.sessionId = sessionID
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
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
            return remoteInfraError(err)
        }

        function toSendTokenResult(
            result: GetResetTokenSendingStatusResult_pb,
        ): ResetTokenSendingResult {
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
        function mapError(
            result: GetResetTokenSendingStatusResult_pb,
        ): CheckResetTokenSendingStatusRemoteError {
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
    }
}
