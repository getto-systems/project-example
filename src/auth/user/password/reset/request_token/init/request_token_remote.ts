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

import { RequestResetTokenRemote } from "../infra"

export function newRequestResetTokenRemote(feature: RemoteOutsideFeature): RequestResetTokenRemote {
    return async (fields) => {
        try {
            const mock = false
            if (mock) {
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/user/password/reset/token",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.auth.user.password.reset.request_token.service.RequestResetTokenRequestPb,
                    (message) => {
                        message.loginId = fields.loginId
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.user.password.reset.request_token.service.RequestResetTokenResponsePb,
                await response.text(),
            )
            if (!message.success) {
                return { success: false, err: { type: "invalid-reset" } }
            }
            return { success: true, value: true }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
