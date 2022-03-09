import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    generateNonce,
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "../infra"

import { ChangePasswordFields } from "../data"

export function newChangePasswordRemote(feature: RemoteOutsideFeature): ChangePasswordRemote {
    return (fields) => fetchRemote(feature, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    fields: ChangePasswordFields,
): Promise<ChangePasswordRemoteResult> {
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
            path: "/auth/user/password",
            method: "PATCH",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.password.change.service.ChangePasswordRequestPb,
                (message) => {
                    message.currentPassword = fields.currentPassword
                    message.newPassword = fields.newPassword
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.password.change.service.ChangePasswordResponsePb,
            await response.text(),
        )
        if (!message.success) {
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
