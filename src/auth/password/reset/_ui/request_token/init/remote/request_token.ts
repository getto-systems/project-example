import { env } from "../../../../../../../y_environment/_ui/env"
import pb from "../../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { RequestResetTokenRemote } from "../../infra"

export function newRequestResetTokenRemote(feature: RemoteOutsideFeature): RequestResetTokenRemote {
    return async (fields) => {
        const RequestResetTokenPb = pb.auth.password.reset.api.RequestResetTokenPb
        const RequestResetTokenResultPb = pb.auth.password.reset.api.RequestResetTokenResultPb

        try {
            const mock = false
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/password/reset/token",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(RequestResetTokenPb, (message) => {
                    message.loginId = fields.loginID
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(RequestResetTokenResultPb, await response.text())
            if (!result.success) {
                return { success: false, err: { type: "invalid-reset" } }
            }
            return { success: true, value: true }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
