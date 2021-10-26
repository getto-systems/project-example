import { env } from "../../../../../../y_environment/ui/env"
import pb from "../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_lib/ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

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
                    pb.auth.user.password.reset.api.RequestResetTokenApiRequestPb,
                    (message) => {
                        message.loginId = fields.loginID
                    },
                ),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(
                pb.auth.user.password.reset.api.RequestResetTokenApiResponsePb,
                await response.text(),
            )
            if (!result.success) {
                return { success: false, err: { type: "invalid-reset" } }
            }
            return { success: true, value: true }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
