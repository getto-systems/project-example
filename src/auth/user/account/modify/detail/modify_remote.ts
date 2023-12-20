import { env } from "../../../../../y_environment/ui/env"
import pb from "../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../common/util/remote/detail/helper"
import { decodeProtobuf, encodeProtobuf } from "../../../../../common/util/protobuf/helper"

import { ModifyAuthUserAccountRemoteResult, ModifyAuthUserAccountRemote } from "../infra"

import { ModifyAuthUserAccountFields, ModifyAuthUserAccountRemoteError } from "../data"
import { LoginId } from "../../../login_id/kernel/data"

export function newModifyAuthUserAccountRemote(): ModifyAuthUserAccountRemote {
    return (user, fields) => fetchRemote(user, fields)
}

async function fetchRemote(
    user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
    fields: ModifyAuthUserAccountFields,
): Promise<ModifyAuthUserAccountRemoteResult> {
    const mock = false
    if (mock) {
        return { success: true, value: true }
    }

    try {
        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/auth/user/account",
            method: "PATCH",
        })
        const response = await fetch(opts.url, {
            ...opts.options,
            body: encodeProtobuf(
                pb.auth.user.account.modify.service.ModifyAuthUserAccountRequestPb,
                (message) => {
                    message.loginId = user.loginId
                    message.from = {
                        ...user,
                        granted: Array.from(user.granted),
                    }
                    message.to = {
                        ...fields,
                        granted: Array.from(fields.granted),
                    }
                },
            ),
        })

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.auth.user.account.modify.service.ModifyAuthUserAccountResponsePb,
            await response.text(),
        )
        if (!message.success) {
            return { success: false, err: errorResponse(message.err) }
        }
        return { success: true, value: true }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function errorResponse(
    err: pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb,
): ModifyAuthUserAccountRemoteError {
    switch (err) {
        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.CONFLICT:
            return { type: "conflict" }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.NOT_FOUND:
            return { type: "not-found" }

        case pb.auth.user.account.modify.service.ModifyAuthUserAccountErrorKindPb.INVALID:
            return { type: "invalid" }
    }
}
