import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../common/util/protobuf/helper"

import { ChangePasswordRemote, ChangePasswordRemoteResult } from "../infra"

import { ChangePasswordFields } from "../data"

export function newChangePasswordRemote(): ChangePasswordRemote {
    return (fields) => fetchRemote(fields)
}

async function fetchRemote(fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult> {
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
