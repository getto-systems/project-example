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

import { ChangePasswordRemoteResult, OverridePasswordRemote } from "../infra"
import { AuthUserAccountBasket } from "../../../account/kernel/data"
import { OverridePasswordFields } from "../data"

export function newOverridePasswordRemote(feature: RemoteOutsideFeature): OverridePasswordRemote {
    return (user, fields) => fetchRemote(feature, user, fields)
}

async function fetchRemote(
    feature: RemoteOutsideFeature,
    user: AuthUserAccountBasket,
    fields: OverridePasswordFields,
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
            path: "/auth/user/password/override",
            method: "PATCH",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.password.change.service.OverridePasswordRequestPb,
                (message) => {
                    message.loginId = user.loginID
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
