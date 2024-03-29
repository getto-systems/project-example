import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../common/util/protobuf/helper"

import { UnregisterAuthUserAccountRemoteResult, UnregisterAuthUserAccountRemote } from "../infra"

import { LoginId } from "../../../login_id/kernel/data"

export function newUnregisterAuthUserAccountRemote(): UnregisterAuthUserAccountRemote {
    return (user) => fetchRemote(user)
}

async function fetchRemote(
    user: Readonly<{ loginId: LoginId }>,
): Promise<UnregisterAuthUserAccountRemoteResult> {
    const mock = false
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "DELETE",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.unregister.service.UnregisterAuthUserAccountRequestPb,
                (message) => {
                    message.loginId = user.loginId
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.unregister.service.UnregisterAuthUserAccountResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return { success: false, err: { type: "invalid" } }
        }
        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}
