import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/init/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../z_vendor/protobuf/helper"

import { ChangePasswordRemoteResult, OverwritePasswordRemote } from "../infra"

import { OverwritePasswordFields } from "../data"
import { LoginId } from "../../../login_id/kernel/data"

export function newOverwritePasswordRemote(): OverwritePasswordRemote {
    return (user, fields) => fetchRemote(user, fields)
}

async function fetchRemote(
    user: Readonly<{ loginId: LoginId }>,
    fields: OverwritePasswordFields,
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
            path: "/auth/user/password/overwrite",
            method: "PATCH",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.password.change.service.OverwritePasswordRequestPb,
                (message) => {
                    message.loginId = user.loginId
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
