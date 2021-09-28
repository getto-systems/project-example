import { env } from "../../../../../../y_environment/ui/env"
import pb from "../../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_lib/ui/remote/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { ChangePasswordRemote } from "../../infra"

export function newChangePasswordRemote(feature: RemoteOutsideFeature): ChangePasswordRemote {
    return async (fields) => {
        const ChangePasswordPb = pb.auth.user.password.api.ChangePasswordPb
        const ChangePasswordResultPb = pb.auth.user.password.api.ChangePasswordResultPb

        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: true,
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/password/change",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(ChangePasswordPb, (message) => {
                    message.currentPassword = fields.currentPassword
                    message.newPassword = fields.newPassword
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(ChangePasswordResultPb, await response.text())
            if (!result.success) {
                return { success: false, err: { type: "invalid-password" } }
            }
            return {
                success: true,
                value: true,
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
